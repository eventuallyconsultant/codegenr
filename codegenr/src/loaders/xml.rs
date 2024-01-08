use super::DocumentLoader;
use quickxml_to_serde::xml_str_to_json;

pub struct XmlLoader {}
impl DocumentLoader for XmlLoader {
  type Error = minidom::Error;
  fn json_from_str(content: &str) -> Result<serde_json::Value, Self::Error> {
    let config = quickxml_to_serde::Config::default();
    xml_str_to_json(content, &config)
  }
}
