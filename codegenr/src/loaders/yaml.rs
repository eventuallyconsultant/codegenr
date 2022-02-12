use super::LoaderError;
use serde_json::{Map, Value};

pub fn yaml_to_json(yaml: serde_yaml::Value) -> Result<Value, LoaderError> {
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
          return Err(LoaderError::YamlToJsonError("Object keys should be strings."));
        }
      }
      Value::Object(json)
    }
  })
}

fn yaml_to_json_number(n: serde_yaml::Number) -> Result<serde_json::Number, LoaderError> {
  use serde_json::Number;
  let number = if n.is_f64() {
    let f = n.as_f64().ok_or(LoaderError::YamlToJsonError("The number should be an f64."))?;
    Number::from_f64(f).ok_or(LoaderError::YamlToJsonError("The number couldn't map to json."))?
  } else if n.is_u64() {
    let u = n.as_u64().ok_or(LoaderError::YamlToJsonError("The number should be an u64."))?;
    Number::from(u)
  } else if n.is_i64() {
    let u = n.as_i64().ok_or(LoaderError::YamlToJsonError("The number should be an i64."))?;
    Number::from(u)
  } else {
    return Err(LoaderError::YamlToJsonError("There is a new number flavor in yaml ?"));
  };
  Ok(number)
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::Map;

  #[test]
  fn yaml_to_json_tests() -> Result<(), LoaderError> {
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
    assert_eq!(e.to_string(), "Couldn't transpile yaml to json : `Object keys should be strings.`.");

    Ok(())
  }

  #[test]
  fn yaml_to_json_number_tests() -> Result<(), anyhow::Error> {
    use serde_yaml::Number;

    let expected_failed_for_f64 = yaml_to_json_number(Number::from(f64::INFINITY));
    let f64_error = expected_failed_for_f64.expect_err("Should be an error");
    assert_eq!(
      f64_error.to_string(),
      "Couldn't transpile yaml to json : `The number couldn't map to json.`."
    );

    let _success_for_f64 = yaml_to_json_number(Number::from(256.2))?;
    let _success_for_u64 = yaml_to_json_number(Number::from(-42))?;
    let _success_for_i64 = yaml_to_json_number(Number::from(42))?;
    let _success_for_neg_value = yaml_to_json_number(Number::from(-40285.5))?;

    Ok(())
  }
}
