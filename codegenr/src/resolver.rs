use serde_json::{Map, Value};

const REF: &str = "$ref";

// https://github.com/BeezUP/dotnet-codegen/tree/master/tests/CodegenUP.DocumentRefLoader.Tests

pub fn load_refs(json: Value /* map<file_name, Value> */) -> Result<Value, anyhow::Error> {
  let json2 = json.clone();
  load_refs_recurse(json, &json2)
}

fn load_refs_recurse(json: Value, original: &Value /* map<file_name, Value> */) -> Result<Value, anyhow::Error> {
  match json {
    Value::Array(_) => {
      todo!();
    }
    Value::Object(obj) => {
      let mut map = Map::new();
      for (key, mut value) in obj.into_iter() {
        if key == REF {
          if let Value::String(val) = value {
            value = resolve_reference(original, &val)?; // #/components/TRUC
          } else {
            return Err(anyhow::anyhow!("{} value should be a String", REF));
          }
        } else {
          value = load_refs_recurse(value, original)?;
        }
        map.insert(key, value);
      }

      Ok(Value::Object(map))
    }
    _ => Ok(json),
  }
}

fn resolve_reference(json: &Value, _path: &str) -> Result<Value, anyhow::Error> {
  Ok(json.clone())
}

#[cfg(test)]
mod test {
  use super::*;
  // use crate::loader::read_json_file;
  use serde_json::json;
 
  #[test]
  fn loading_refs_test() -> Result<(), anyhow::Error> {
    // Verif structure + pretty print Json : https://jsonformatter.org/json-pretty-print
    let json = json!({
      "test": { "$ref": "#/myref" },
      "myref": { "data": "test" }
    });

    let expected = json!({
      "test": { "data": "test", "x-fromRef": "#/myref", "x-refName": "myref" },
      "myref": { "data": "test" }
    });

    let loaded = load_refs(json)?;
    println!("{}", loaded.to_string());
    println!("{}", expected.to_string());
    assert_eq!(loaded, expected);
    Ok(())
  }
  
  #[test]
  fn loading_refs_test_2() -> Result<(), anyhow::Error> {
    let json = json!({
      "test": { "data1": { "$ref": "#/myref" }, "data2": { "$ref": "#/myref" }},
      "myref": { "data": "test" }
    });

    let expected = json!({
      "test": { "data1": { "data": "test", "x-fromRef": "#/myref", "x-refName": "myref" }, "data2": { "data": "test", "x-fromRef": "#/myref", "x-refName": "myref" }},
      "myref": { "data": "test" }
    });

    let loaded = load_refs(json)?;
    println!("{}", loaded.to_string());
    println!("{}", expected.to_string());
    assert_eq!(loaded, expected);
    Ok(())
  }

  #[test]
  fn loading_refs_test_3() -> Result<(), anyhow::Error> {
    let json = json!({
      "test": { "data1": { "$ref": "#/myref" }, "data2": { "$ref": "#/myref" }},
      "myref": { "data": { "$ref": "#/myref2" }},
      "myref2": { "content": { "data": "test"}}
    });

    let expected = json!({
      "test": { "data1": { "data": { "content": { "data": "test" }, "x-fromRef": "#/myref2", "x-refName": "myref2"}, "x-fromRef":"#/myref", "x-refName": "myref" },
                "data2": { "data": { "content": { "data": "test" }, "x-fromRef": "#/myref2", "x-refName": "myref2"}, "x-fromRef":"#/myref", "x-refName": "myref" }},
      "myref": { "data": { "content": { "data": "test" }, "x-fromRef": "#/myref2", "x-refName":"myref2" } },
      "myref2": { "content": { "data": "test"} }
    });

    let loaded = load_refs(json)?;
    println!("{}", loaded.to_string());
    println!("{}", expected.to_string());
    assert_eq!(loaded, expected);
    Ok(())
  }

  // TODO: last test
  //             yield return new[] // works with json
  //            {
  // @"{
  //    ""test"": {
  //       ""data1"": {
  //          ""$ref"": ""#/myref""
  //       },
  //       ""data2"": {
  //          ""$ref"": ""#/myref""
  //       }
  //    },
  //    ""myref"": {
  //       ""data"": {
  //          ""$ref"": ""#/myref2""
  //       }
  //    },
  //    ""myref2"": {
  //       ""content"": {
  //          ""data"": ""test""
  //       }
  //    }
  // }
  // ",
  // @"test:
  //   data1:
  //     data:
  //       content:
  //         data: ""test""
  //       x-fromRef: ""#/myref2""
  //       x-refName: ""myref2""
  //     x-fromRef: ""#/myref""
  //     x-refName: ""myref""
  //   data2:
  //     data:
  //       content:
  //         data: ""test""
  //       x-fromRef: ""#/myref2""
  //       x-refName: ""myref2""
  //     x-fromRef: ""#/myref""
  //     x-refName: ""myref""
  // myref:
  //   data:
  //     content:
  //       data: ""test""
  //     x-fromRef: ""#/myref2""
  //     x-refName: ""myref2""
  // myref2:
  //   content:
  //     data: ""test""
  // "
  //             };
}
