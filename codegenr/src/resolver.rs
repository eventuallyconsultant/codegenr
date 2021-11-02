use serde_json::{Map, Value};
use crate::loader::read_json_file;

const REF: &str = "$ref";

// https://github.com/BeezUP/dotnet-codegen/tree/master/tests/CodegenUP.DocumentRefLoader.Tests

fn load_refs(json: Value /* map<file_name, Value> */) -> Result<Value, anyhow::Error> {
  match json {
    Value::Array(_) => {
      todo!();
    }
    Value::Object(obj) => {
      let mut map = Map::new();
      for (key, value) in obj.into_iter() {
        map.insert(key, value);
         if key == REF {
           if let Value::String(value) = value {
              let ref_value = value;
              //value = resolve_reference();
            } else {
              panic!("Should be a String");
           }
         }
      }

      Ok(Value::Object(map))
    }
    _ => Ok(json),
  }
}

fn resolve_reference() -> Result<Value, anyhow::Error> {
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