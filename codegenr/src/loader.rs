use serde_json::{Map, Value};
use std::{fs::File, io::Read};

pub fn read_json_file(file_path: &str) -> Result<Value, anyhow::Error> {
  let mut file = File::open(file_path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(serde_json::from_str(&contents)?)
}

pub fn read_yaml_file(file_path: &str) -> Result<Value, anyhow::Error> {
  let mut file = File::open(file_path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let yaml: serde_yaml::Value = serde_yaml::from_str(&contents)?;
  yaml_to_json(yaml)
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

  #[test]
  fn read_yaml_file_test() -> Result<(), anyhow::Error> {
    let result = read_yaml_file("./_samples/Merge1.yaml")?;
    dbg!(result);
    Ok(())
  }

  #[test]
  fn read_json_file_test() -> Result<(), anyhow::Error> {
    let result = read_json_file("./_samples/Merge1_rest.json")?;
    dbg!(result);
    Ok(())
  }

  #[test]
  fn yaml_to_json_tests() -> Result<(), anyhow::Error> {
    use serde_yaml::Value::*;
    assert_eq!(yaml_to_json(Null)?, Value::Null);
    assert_eq!(yaml_to_json(Bool(true))?, Value::Bool(true));
    assert_eq!(yaml_to_json(Bool(false))?, Value::Bool(false));
    assert_eq!(yaml_to_json(String("test".into()))?, Value::String("test".into()));
    //assert_eq!(yaml_to_json(Number)?, Value::Number());
    // todo : Number (above)

    assert_eq!(
      yaml_to_json(Sequence(vec!(Null, Bool(true), String("test".into()))))?,
      Value::Array(vec!(Value::Null, Value::Bool(true), Value::String("test".into())))
    );

    let mut map = serde_yaml::Mapping::new();
    map.insert(String("key".into()), String("value".into()));
    let mut expected = Map::new();
    expected.insert("key".into(), Value::String("value".into()));

    assert_eq!(yaml_to_json(Mapping(map))?, Value::Object(expected));
    // yaml_to_json(Null).map_err(|e| e.to_string());
    
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

    let expected_failed = yaml_to_json_number(Number::from(f64::INFINITY));
    let e = expected_failed.expect_err("Should be an error");
    assert_eq!(e.to_string(), "The number couldn't map to json.");

    // todo : ok tests with  Number::from() i64, u64, and non weird f64 ... and some negative values
    //let expected = yaml_to_json_number(Number::from(i64::abs(42)));

    Ok(())
  }
}
