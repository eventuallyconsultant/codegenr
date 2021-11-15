use path_dedot::ParseDot;
use serde_json::{Map, Value};
use std::{fs::File, io::Read, path::Path};
use url::Url;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum DocumentPath {
  /// Full url to a file : https://mywebsite/api.yaml
  Url(Url),
  /// File name or relative file name
  FileName(String),
  /// json or yaml out of thin silicon
  None,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub(crate) enum FormatHint {
  /// The content should be json
  Json,
  /// The content should be yaml
  Yaml,
  /// We have no f.....g idea
  NoIdea,
}

impl DocumentPath {
  pub fn parse(ref_path: &str) -> Result<Self, anyhow::Error> {
    Ok(if ref_path.trim() == "" {
      Self::None
    } else {
      match Url::parse(ref_path) {
        Ok(url) => DocumentPath::Url(url),
        Err(_) => DocumentPath::FileName(ref_path.into()),
      }
    })
  }

  pub fn relate_from(self, refed_from: &Self) -> Result<Self, anyhow::Error> {
    use DocumentPath::*;
    Ok(match (refed_from, self) {
      (Url(_), Url(url)) => Url(url),
      (Url(url_from), FileName(path_to)) => {
        let mut url = url_from.clone();
        url.path_segments_mut().map_err(|_| anyhow::anyhow!("Url cannot be a base."))?.pop();
        let path = url.path();
        let new_path = Path::new(path).join(&path_to);
        let new_path = new_path.parse_dot()?;
        let new_path = new_path
          .to_str()
          .ok_or_else(|| anyhow::anyhow!("Unable to append path '{}' to '{}'", path_to, url_from))?;
        url.set_path(new_path);
        Url(url)
      }
      (Url(_), None) => refed_from.clone(),
      (FileName(path_from), FileName(path_to)) => {
        let folder = Path::new(path_from)
          .parent()
          .ok_or_else(|| anyhow::anyhow!("The origin path should be a file and have parent."))?;
        folder
          .join(&path_to)
          .parse_dot()?
          .to_str()
          .map(|s| FileName(s.to_owned()))
          .ok_or_else(|| anyhow::anyhow!("Unable to append path '{}' to '{}'", path_to, path_from))?
      }
      (FileName(_), Url(url)) => Url(url),
      (FileName(_path_from), None) => refed_from.clone(),
      (None, s) => s,
    })
  }

  pub(crate) fn guess_format(&self) -> FormatHint {
    let s = match self {
      DocumentPath::Url(url) => url.as_str(),
      DocumentPath::FileName(s) => s,
      DocumentPath::None => return FormatHint::NoIdea,
    };
    if s.ends_with(".json") {
      FormatHint::Json
    } else if s.ends_with(".yaml") || s.ends_with(".yml") {
      FormatHint::Yaml
    } else {
      FormatHint::NoIdea
    }
  }

  pub fn load_raw(&self) -> Result<Value, anyhow::Error> {
    let hint = self.guess_format();
    match self {
      DocumentPath::Url(url) => {
        let body = reqwest::blocking::get(url.clone())?.text()?;
        json_from_string(&body, hint)
      }
      DocumentPath::FileName(file_name) => {
        let mut file = File::open(file_name)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        json_from_string(&content, hint)
      }
      DocumentPath::None => unreachable!("This is a non sense to try loading a 'None' document path."),
    }
  }
}

fn json_from_string(content: &str, hint: FormatHint) -> Result<Value, anyhow::Error> {
  match hint {
    FormatHint::Json | FormatHint::NoIdea => {
      let json_error = match serde_json::from_str(content) {
        Ok(json) => return Ok(json),
        Err(e) => e,
      };

      let yaml_error = match serde_yaml::from_str(content) {
        Ok(yaml) => return yaml_to_json(yaml),
        Err(e) => e,
      };

      Err(anyhow::anyhow!(
        "Could not read file content as json:\n-json_error: '{}'\n-yaml_error:'{}'",
        json_error,
        yaml_error,
      ))
    }
    FormatHint::Yaml => {
      let yaml_error = match serde_yaml::from_str(content) {
        Ok(yaml) => return yaml_to_json(yaml),
        Err(e) => e,
      };

      let json_error = match serde_json::from_str(content) {
        Ok(json) => return Ok(json),
        Err(e) => e,
      };

      Err(anyhow::anyhow!(
        "Could not read file content as json:\n-yaml_error:'{}'\n-json_error: '{}'",
        yaml_error,
        json_error,
      ))
    }
  }
}

fn yaml_to_json(yaml: serde_yaml::Value) -> Result<Value, anyhow::Error> {
  use serde_yaml::Value::*;
  Ok(match yaml {
    Null => Value::Null,
    Bool(b) => Value::Bool(b),
    Number(n) => Value::Number(yaml_to_json_number(n)?),
    String(s) => Value::String(s),
    Sequence(values) => Value::Array(values.into_iter().map(yaml_to_json).collect::<Result<Vec<_>, _>>()?),
    Mapping(map) => {
      let mut json = Map::<_, _>::with_capacity(map.len());
      for (key, value) in map {
        if let String(s) = key {
          json.insert(s, yaml_to_json(value)?);
        } else {
          return Err(anyhow::anyhow!("Object keys should be strings."));
        }
      }
      Value::Object(json)
    }
  })
}

fn yaml_to_json_number(n: serde_yaml::Number) -> Result<serde_json::Number, anyhow::Error> {
  use serde_json::Number;
  let number = if n.is_f64() {
    let f = n.as_f64().ok_or_else(|| anyhow::format_err!("The number should be an f64."))?;
    Number::from_f64(f).ok_or_else(|| anyhow::format_err!("The number couldn't map to json."))?
  } else if n.is_u64() {
    let u = n.as_u64().ok_or_else(|| anyhow::format_err!("The number should be an u64."))?;
    Number::from(u)
  } else if n.is_i64() {
    let u = n.as_i64().ok_or_else(|| anyhow::format_err!("The number should be an i64."))?;
    Number::from(u)
  } else {
    return Err(anyhow::anyhow!("There is a new number flavor in yaml ?"));
  };
  Ok(number)
}

#[cfg(test)]
mod test {
  use super::*;
  use test_case::test_case;

  #[test_case("h://f", "h://f", "h://f")]
  #[test_case("h://w.com/api.yaml", "components.yaml", "h://w.com/components.yaml")]
  #[test_case("h://w.com/v1/api.yaml", "../v2/components.yaml", "h://w.com/v2/components.yaml")]
  #[test_case("file.yaml", "other.json", "other.json")]
  #[test_case("test/file.yaml", "other.json", "test/other.json")]
  #[test_case("test/file.yaml", "./other2.json", "test/other2.json")]
  #[test_case("test/file.yaml", "../other3.json", "other3.json")]
  #[test_case("test/file.yaml", "plop/other.json", "test/plop/other.json")]
  #[test_case("file.yaml", "http://w.com/other.json", "http://w.com/other.json")]
  #[test_case("file.json", "", "file.json")]
  #[test_case("", "f", "f")]
  #[test_case("", "h://f", "h://f")]
  #[test_case("_samples/petshop_with_external.yaml", "petshop_externals.yaml", "_samples/petshop_externals.yaml")]
  // #[test_case(
  //   "./_samples/petshop_with_external.yaml",
  //   "petshop_externals.yaml",
  //   "./_samples/petshop_externals.yaml"
  // )]
  fn relate_test(doc_path: &str, ref_path: &str, expected_related: &str) {
    let doc_path = DocumentPath::parse(doc_path).expect("?");
    let r_path = DocumentPath::parse(ref_path).expect("?");
    let expected_related = DocumentPath::parse(expected_related).expect("?");
    let related = r_path.relate_from(&doc_path).expect("?");
    assert_eq!(related, expected_related);
  }

  #[test]
  fn read_json_file_test() -> Result<(), anyhow::Error> {
    let _result = DocumentPath::parse("./_samples/Merge1_rest.json")?.load_raw()?;
    Ok(())
  }

  #[test]
  fn read_yaml_file_test() -> Result<(), anyhow::Error> {
    let _result = DocumentPath::parse("./_samples/Merge1.yaml")?.load_raw()?;
    Ok(())
  }

  #[test]
  fn read_beezup_openapi() -> Result<(), anyhow::Error> {
    let _result = DocumentPath::parse("https://api-docs.beezup.com/swagger.json")?.load_raw()?;
    Ok(())
  }

  #[test]
  fn yaml_to_json_tests() -> Result<(), anyhow::Error> {
    use serde_yaml::Value::*;
    assert_eq!(yaml_to_json(Null)?, Value::Null);
    assert_eq!(yaml_to_json(Bool(true))?, Value::Bool(true));
    assert_eq!(yaml_to_json(Bool(false))?, Value::Bool(false));
    assert_eq!(yaml_to_json(String("test".into()))?, Value::String("test".into()));
    assert_eq!(
      yaml_to_json(Number(serde_yaml::from_str("2")?))?,
      Value::Number(serde_json::from_str("2")?)
    );

    assert_eq!(
      yaml_to_json(Sequence(vec!(Null, Bool(true), String("test".into()))))?,
      Value::Array(vec!(Value::Null, Value::Bool(true), Value::String("test".into())))
    );

    let mut map = serde_yaml::Mapping::new();
    map.insert(String("key".into()), String("value".into()));
    let mut expected = Map::new();
    expected.insert("key".into(), Value::String("value".into()));

    assert_eq!(yaml_to_json(Mapping(map))?, Value::Object(expected));

    let mut map = serde_yaml::Mapping::new();
    map.insert(Null, String("value".into()));
    let expected_failed = yaml_to_json(Mapping(map));
    let e = expected_failed.expect_err("Should be an error");
    assert_eq!(e.to_string(), "Object keys should be strings.");

    Ok(())
  }

  #[test]
  fn yaml_to_json_number_tests() -> Result<(), anyhow::Error> {
    use serde_yaml::Number;

    let expected_failed_for_f64 = yaml_to_json_number(Number::from(f64::INFINITY));
    let f64_error = expected_failed_for_f64.expect_err("Should be an error");
    assert_eq!(f64_error.to_string(), "The number couldn't map to json.");

    let _success_for_f64 = yaml_to_json_number(Number::from(256.2))?;
    let _success_for_u64 = yaml_to_json_number(Number::from(-42))?;
    let _success_for_i64 = yaml_to_json_number(Number::from(42))?;
    let _success_for_neg_value = yaml_to_json_number(Number::from(-40285.5))?;

    Ok(())
  }
}
