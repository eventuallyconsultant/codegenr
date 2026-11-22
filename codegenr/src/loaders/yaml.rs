use super::DocumentLoader;
use serde_json::Value;

pub struct YamlLoader {}
impl DocumentLoader for YamlLoader {
  type Error = serde_yaml::Error;
  fn json_from_str(content: &str) -> Result<Value, Self::Error> {
    serde_yaml::from_str(content)
  }
}
