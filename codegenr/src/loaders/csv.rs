use super::DocumentLoader;
use serde_json::Value;

pub struct CsvLoader {}
impl DocumentLoader for CsvLoader {
  type Error = csv::Error;
  fn json_from_str(content: &str) -> Result<Value, Self::Error> {
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let lines = rdr
      .records()
      .map(|result| {
        let record = result?;
        Ok(Value::Object(
          record
            .into_iter()
            .enumerate()
            .map(|(i, field)| (format!("field_{i}"), Value::String(field.into())))
            .collect(),
        ))
      })
      .collect::<Result<Vec<_>, Self::Error>>()?;
    Ok(Value::Array(lines))
  }
}
