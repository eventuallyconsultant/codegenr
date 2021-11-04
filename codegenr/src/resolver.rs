use serde_json::{Map, Value};

const REF: &str = "$ref";
const PATH_SEP: char = '/';
const FROM_REF: &str = "x-fromRef";
const REF_NAME: &str = "x-refName";

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
          if let Value::String(path) = value {
            map.insert(FROM_REF.into(), Value::String(path.clone()));
            //map.insert(REF_NAME.into(), get_ref_name(&path) );
            value = resolve_reference(original, &path)?; // #/components/TRUC
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

// /test/ezgliuh/value -> value
// split la path et récup la dernière
fn get_ref_name(path: &str) -> String {
  let split_value = path.split("/").last();
  todo!()
}

fn resolve_reference(json: &Value, path: &str) -> Result<Value, anyhow::Error> {
  let parts = path.split(PATH_SEP);

  let mut part = json;

  for p in parts.filter(|p| *p != "#") {
    if let Value::Object(o) = part {
      part = o
        .get(p)
        .ok_or_else(|| anyhow::format_err!("Key {} was not found in json part {}", p, part))?;
    } else {
      return Err(anyhow::anyhow!("Could not follow path {} as json part is not an object.", p));
    }
  }

  Ok(part.clone())
}

#[cfg(test)]
mod test {
  use super::*;
  // use crate::loader::read_json_file;
  use serde_json::json;

  #[test]
  fn resolve_reference_test() -> Result<(), anyhow::Error> {
    let json = json!({
      "test": {
        "data1": {
          "value": 42
        },
        "data2": [
          1,2,3
        ]
      },
      "myref": {
        "data": "test"
      }
    });

    assert_eq!(
      resolve_reference(&json, "#/test/data1/value")?,
      Value::Number(serde_json::Number::from(42))
    );

    assert_eq!(resolve_reference(&json, "#/test/data1")?, json!({ "value": 42 }));

    // todo : error case
    let path: &str = "#/test/not_existing_path";
    let failed_test = resolve_reference(&json, path);
    let err = failed_test.expect_err("Should be an error");
    assert_eq!(err.to_string(), "Key not_existing_path was not found in json part {\"data1\":{\"value\":42},\"data2\":[1,2,3]}");

    Ok(())
  }

  #[test]
  fn loading_refs_test() -> Result<(), anyhow::Error> {
    // Verif structure + pretty print Json : https://jsonformatter.org/json-pretty-print
    let json = json!({
      "test": {
        "$ref": "#/myref"
      },
      "myref": {
        "data": "test"
      }
    });

    let expected = json!({
      "test": {
        "data": "test",
        "x-fromRef": "#/myref",
        "x-refName": "myref"
      },
      "myref": {
        "data": "test"
      }
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
      "test": {
        "data1": {
          "$ref": "#/myref"
        },
        "data2": {
          "$ref": "#/myref"
        }
      },
      "myref": {
        "data": "test"
      }
    });

    let expected = json!({
      "test": {
        "data1": {
          "data": "test",
          "x-fromRef": "#/myref",
          "x-refName": "myref"
        },
        "data2": {
          "data": "test",
          "x-fromRef": "#/myref",
          "x-refName": "myref"
        }
      },
      "myref": {
        "data": "test"
      }
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
      "test": {
        "data1": {
          "$ref": "#/myref"
        },
        "data2": {
          "$ref": "#/myref"
        }
      },
      "myref": {
        "data": {
          "$ref": "#/myref2"
        }
      },
      "myref2": {
        "content": {
          "data": "test"
        }
      }
    });

    let expected = json!({
      "test": {
        "data1": {
          "data": {
            "content": {
              "data": "test"
            },
            "x-fromRef": "#/myref2",
            "x-refName": "myref2"
          },
          "x-fromRef": "#/myref",
          "x-refName": "myref"
        },
        "data2": {
          "data": {
            "content": {
              "data": "test"
            },
            "x-fromRef": "#/myref2",
            "x-refName": "myref2"
          },
          "x-fromRef": "#/myref",
          "x-refName": "myref"
        }
      },
      "myref": {
        "data": {
          "content": {
            "data": "test"
          },
          "x-fromRef": "#/myref2",
          "x-refName": "myref2"
        }
      },
      "myref2": {
        "content": {
          "data": "test"
        }
      }
    });

    let loaded = load_refs(json)?;
    println!("{}", loaded.to_string());
    println!("{}", expected.to_string());
    assert_eq!(loaded, expected);
    Ok(())
  }

  #[test]
  fn loading_refs_test_4() -> Result<(), anyhow::Error> {
    let json = json!({
        "test": {
          "data1": {
            "$ref": "#/myref"
          },
          "data2": {
            "$ref": "#/myref"
          }
        },
        "myref": {
          "data": {
            "$ref": "#/myref2"
          }
        },
        "myref2": {
          "content": {
            "data": "test"
          }
        }
    });

    let expected = json!({
        "test": {
          "data1": {
            "data": {
              "content": {
                "data": "test"
              },
              "x-fromRef": "#/myref2",
              "x-refName": "myref2"
            },
            "x-fromRef": "#/myref",
            "x-refName": "myref"
          },
          "data2": {
            "data": {
              "content": {
                "data": "test"
              },
              "x-fromRef": "#/myref2",
              "x-refName": "myref2"
            },
            "x-fromRef": "#/myref",
            "x-refName": "myref"
          },
          "myref": {
            "data": {
              "content": {
                "data": "test"
              },
              "x-fromRef": "#/myref2",
              "x-refName": "myref2"
            },
            "myref2": {
              "content": {
                "data": "test"
              }
            }
          }
        }
    });

    let loaded = load_refs(json)?;
    println!("{}", loaded.to_string());
    println!("{}", expected.to_string());
    assert_eq!(loaded, expected);
    Ok(())
  }
}
