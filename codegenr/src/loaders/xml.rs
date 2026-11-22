use super::DocumentLoader;
use roxmltree::{Document, Node, ParsingOptions};
use serde_json::{Map, Number, Value};

const ATTRIBUTE_PREFIX: &str = "@";
const TEXT_KEY: &str = "#text";

/// Turns an xml document into json with the rules of `quickxml_to_serde` 0.6 and its default config, the library this loader used before:
/// `{ "<root>": … }`, the attributes as `@name` keys sorted by name, the text of an element that also has attributes as `#text`,
/// repeated elements as an array, an empty element as `{}`, and the values typed (numbers, booleans) when they parse as such.
/// An element with some text keeps only its text and its attributes: its child elements are dropped.
/// `_samples/loader/xml_shapes.json` shows each of these shapes.
pub struct XmlLoader {}
impl DocumentLoader for XmlLoader {
  type Error = roxmltree::Error;
  fn json_from_str(content: &str) -> Result<Value, Self::Error> {
    // A DTD was accepted before, so it still is; roxmltree guards against the billion laughs attack anyway.
    let options = ParsingOptions {
      allow_dtd: true,
      ..Default::default()
    };
    let document = Document::parse_with_options(content, options)?;
    let root = document.root_element();
    let mut json = Map::new();
    json.insert(root.tag_name().name().to_string(), element_to_json(root));
    Ok(Value::Object(json))
  }
}

fn element_to_json(element: Node) -> Value {
  let text: String = element.children().filter(Node::is_text).filter_map(|node| node.text()).collect();
  let mut json = attributes_to_json(element);

  if !text.trim().is_empty() {
    if json.is_empty() {
      return typed_value(&text);
    }
    json.insert(TEXT_KEY.into(), typed_value(&text));
    return Value::Object(json);
  }

  for child in element.children().filter(Node::is_element) {
    let name = child.tag_name().name().to_string();
    let value = element_to_json(child);
    match json.get_mut(&name) {
      Some(Value::Array(values)) => values.push(value),
      // Removed then inserted back as before: the `swap_remove` of serde_json moves the last key in its place,
      // and the array goes to the end (`apart` in `xml_shapes.json`).
      _ => {
        let value = match json.remove(&name) {
          Some(first) => Value::Array(vec![first, value]),
          None => value,
        };
        json.insert(name, value);
      }
    }
  }
  Value::Object(json)
}

fn attributes_to_json(element: Node) -> Map<String, Value> {
  let mut attributes: Vec<(String, Value)> = element
    .attributes()
    .map(|attribute| {
      let name = match attribute.namespace().and_then(|uri| element.lookup_prefix(uri)) {
        Some(prefix) => format!("{ATTRIBUTE_PREFIX}{prefix}:{}", attribute.name()),
        None => format!("{ATTRIBUTE_PREFIX}{}", attribute.name()),
      };
      (name, typed_value(attribute.value()))
    })
    .collect();
  attributes.sort_by(|(a, _), (b, _)| a.cmp(b));
  attributes.into_iter().collect()
}

/// `42` and `12.50` become numbers, `true` a boolean; a number with a leading zero (`01.5`) stays a string, except an integer (`007` gives `7`).
fn typed_value(text: &str) -> Value {
  let text = text.trim();
  if let Ok(integer) = text.parse::<u64>() {
    return Value::Number(integer.into());
  }
  if let Ok(float) = text.parse::<f64>() {
    if text.starts_with('0') && !text.starts_with("0.") {
      return Value::String(text.into());
    }
    if let Some(number) = Number::from_f64(float) {
      return Value::Number(number);
    }
  }
  if let Ok(boolean) = text.parse::<bool>() {
    return Value::Bool(boolean);
  }
  Value::String(text.into())
}
