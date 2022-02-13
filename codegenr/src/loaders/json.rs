use super::{DocumentLoader, LoaderError};
use serde_json::Value;

pub struct JsonLoader {}
impl DocumentLoader for JsonLoader {
  type Error = serde_json::Error;
  fn json_from_str(content: &str) -> Result<Value, Self::Error> {
    Ok(serde_json::from_str(content)?)
  }
}
