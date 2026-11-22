use super::DocumentLoader;
use serde_json::Value;

pub struct TomlLoader {}
impl DocumentLoader for TomlLoader {
  type Error = toml::de::Error;
  fn json_from_str(content: &str) -> Result<Value, Self::Error> {
    toml::from_str(content)
  }
}
