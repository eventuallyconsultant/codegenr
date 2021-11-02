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
      for (key, value) in obj.into_iter() {
        map.insert(key, value);
         if key == REF {
           if let String(value)  = value {
              let ref_value = value;
              value = resolve_reference();
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
