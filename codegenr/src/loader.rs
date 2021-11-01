use serde_json::Value;
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

  todo!("pouet!")
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
