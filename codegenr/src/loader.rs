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
    Number(n) => Value::Number(yaml_to_json_number(n)),
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
