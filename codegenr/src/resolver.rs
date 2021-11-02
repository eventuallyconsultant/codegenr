use crate::loader::read_json_file;
use serde_json::{Map, Value};

const REF: &str = "$ref";

// https://github.com/BeezUP/dotnet-codegen/tree/master/tests/CodegenUP.DocumentRefLoader.Tests

fn load_refs(json: Value /* map<file_name, Value> */) -> Result<Value, anyhow::Error> {
  match json {
    Value::Array(_) => {
      todo!();
    }
    Value::Object(obj) => {
      let mut map = Map::new();
      for (key, mut value) in obj.into_iter() {
        if key == REF {
          if let Value::String(val) = value {
            value = resolve_reference(&json, &val)?; // #/components/TRUC
          } else {
            panic!("Should be a String");
          }
        };
        map.insert(key, value);
      }

      Ok(Value::Object(map))
    }
    _ => Ok(json),
  }
}

fn resolve_reference(json: &Value, path: &str) -> Result<Value, anyhow::Error> {
  todo!();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn loading_refs_test() -> Result<(), anyhow::Error> {
    let read = read_json_file("./_samples/simple2.json")?;
    let result = load_refs(read);
    Ok(())
  }
}
