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
  Ok(match yaml {
    serde_yaml::Value::Null => Value::Null,
    serde_yaml::Value::Bool(b) => Value::Bool(b),
    serde_yaml::Value::Number(n) => Value::Number(yaml_to_json_number(n)),
    serde_yaml::Value::String(s) => Value::String(s),
    serde_yaml::Value::Sequence(values) => Value::Array(values.into_iter().map(yaml_to_json).collect::<Result<Vec<_>, _>>()?),
    serde_yaml::Value::Mapping(map) => {
      let mut json = Map::<String, Value>::new();
      for (key, value) in map {
        if let serde_yaml::Value::String(s) = key {
          json.insert(s, yaml_to_json(value)?);
        } else {
          return Err(anyhow::anyhow!("Object keys should be strings."));
        }
      }
      Value::Object(json)
    }
  })
}

fn yaml_to_json_number(_number: serde_yaml::Number) -> serde_json::Number {
  todo!()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn read_yaml_file_test() {
    let result = read_yaml_file("./_samples/Merge1.yaml").expect("?");
  }

  #[test]
  fn read_json_file_test() -> Result<(), anyhow::Error> {
    let result = read_json_file("./_samples/Merge1_rest.json")?;
    Ok(())
  }
}
