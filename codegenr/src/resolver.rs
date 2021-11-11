use crate::loader::DocumentPath;
use serde_json::{Map, Value};
use std::collections::HashMap;

const REF: &str = "$ref";
const PATH_SEP: char = '/';
const FROM_REF: &str = "x-fromRef";
const REF_NAME: &str = "x-refName";

type DocumentsHash = HashMap<DocumentPath, Value>;

pub struct RefResolver {
  hash: DocumentsHash,
}

// impl RefResolver {
//   pub fn resolve_from_value(json: Value) -> Result<Value, anyhow::Error> {
//     todo!()
//   }

//   pub fn resolve_document(document_path: &str) -> Result<Value, anyhow::Error> {
//     todo!()
//   }
// }

// https://github.com/BeezUP/dotnet-codegen/tree/master/tests/CodegenUP.DocumentRefLoader.Tests

pub fn resolve_refs_raw(json: Value) -> Result<Value, anyhow::Error> {
  let json2 = json.clone();
  resolve_refs_recurse(json, &json2, &mut Default::default())
}

pub fn resolve_refs(document: DocumentPath) -> Result<Value, anyhow::Error> {
  todo!()
}

fn resolve_refs_recurse(json: Value, original: &Value, cache: &mut DocumentsHash) -> Result<Value, anyhow::Error> {
  match json {
    Value::Array(a) => {
      let mut new = Vec::<_>::with_capacity(a.len());
      for v in a {
        new.push(resolve_refs_recurse(v, original, cache)?);
      }
      Ok(Value::Array(new))
    }
    Value::Object(obj) => {
      let mut map = Map::new();
      for (key, value) in obj.into_iter() {
        if key != REF {
          map.insert(key, resolve_refs_recurse(value, original, cache)?);
        } else if let Value::String(path) = value {
          let new = resolve_reference(original, &path)?;
          match new {
            Value::Object(m) => {
              for (k, v) in m {
                map.insert(k, resolve_refs_recurse(v, original, cache)?);
              }
              map.insert(FROM_REF.into(), Value::String(path.clone()));
              map.insert(REF_NAME.into(), Value::String(get_ref_name(&path)));
            }
            v => return resolve_refs_recurse(v, original, cache),
          }
        } else {
          return Err(anyhow::anyhow!("{} value should be a String", REF));
        }
      }

      Ok(Value::Object(map))
    }
    _ => Ok(json),
  }
}

// /test/ezgliuh/value -> value
// split la path et récup la dernière
fn get_ref_name(path: &str) -> String {
  path.split('/').last().unwrap_or_default().to_string()
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

#[derive(Debug)]
pub struct RefInfo {
  /// Path of the reference to import in the destination file
  pub path: Option<String>,
  /// True if the reference is nested in the same document
  pub is_nested: bool,
  /// File path of the document containing the reference
  pub document_path: DocumentPath,
  /// Last part of the $ref value
  pub ref_friendly_name: Option<String>,
  // pub abs_doc_uri: Url,
  // pub is_false_abs_ref: bool,
  // public Uri AbsoluteDocumentUri { get; }
}

impl RefInfo {
  pub fn parse(doc_path: &DocumentPath, ref_value: &str) -> Result<Self, anyhow::Error> {
    let mut parts = ref_value.split('#');

    let (ref_doc_path, path) = match (parts.next(), parts.next(), parts.next()) {
      (_, _, Some(_)) => {
        return Err(anyhow::anyhow!(
          "There should be no more than 2 parts separated by # in a reference path."
        ))
      }
      (Some(file), None, None) => (DocumentPath::parse(file)?.relate_from(doc_path)?, None),
      (Some(""), Some(p), None) => (doc_path.clone(), Some(p.to_string())),
      (Some(file), Some(p), None) => (DocumentPath::parse(file)?.relate_from(doc_path)?, Some(p.to_string())),
      (None, _, _) => unreachable!("Split always returns at least one element"),
    };

    let is_nested: bool = doc_path == &ref_doc_path;
    let ref_friendly_name = path.as_ref().map(|p| p.split('/').last().unwrap_or_default().to_string());

    Ok(Self {
      path,
      is_nested,
      document_path: ref_doc_path,
      ref_friendly_name,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::loader::*;
  use serde_json::json;
  use test_case::test_case;

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

    let path: &str = "#/test/not_existing_path";
    let failed_test = resolve_reference(&json, path);
    let err = failed_test.expect_err("Should be an error");
    assert_eq!(
      err.to_string(),
      "Key not_existing_path was not found in json part {\"data1\":{\"value\":42},\"data2\":[1,2,3]}"
    );

    Ok(())
  }

  #[test]
  fn resolve_refs_test() -> Result<(), anyhow::Error> {
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
        "x-refName": "myref",
      },
      "myref": {
        "data": "test"
      }
    });

    let resolved = resolve_refs_raw(json)?;
    println!("{}", resolved.to_string());
    println!("{}", expected.to_string());
    assert_eq!(resolved, expected);
    Ok(())
  }

  #[test]
  fn resolve_refs_test_2() -> Result<(), anyhow::Error> {
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

    let resolved = resolve_refs_raw(json)?;
    println!("{}", resolved.to_string());
    println!("{}", expected.to_string());
    assert_eq!(resolved, expected);
    Ok(())
  }

  #[test]
  fn resolve_refs_test_3() -> Result<(), anyhow::Error> {
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

    let resolved = resolve_refs_raw(json)?;
    println!("{}", resolved.to_string());
    println!("{}", expected.to_string());
    assert_eq!(resolved, expected);
    Ok(())
  }

  #[test]
  fn resolve_refs_test_4() -> Result<(), anyhow::Error> {
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

    let resolved = resolve_refs_raw(json)?;
    println!("{}", resolved.to_string());
    println!("{}", expected.to_string());
    assert_eq!(resolved, expected);
    Ok(())
  }

  #[test]
  fn should_resolve_nested_references() -> Result<(), anyhow::Error> {
    let json = DocumentPath::parse("./_samples/petshop.yaml")?.load_raw()?;
    let json = resolve_refs_raw(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[test]
  fn should_resolve_external_references() -> Result<(), anyhow::Error> {
    let json = DocumentPath::parse("./_samples/petshop_with_external.yaml")?.load_raw()?;
    let json = resolve_refs_raw(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[rustfmt::skip]
  #[test_case("", "", true, "", None, None)]
  #[test_case("_samples/petshop.yaml", "../test.json", false, "test.json", None, None)]
  #[test_case("_samples/petshop.yaml", "test.json", false, "_samples/test.json", None, None)]
  #[test_case("_samples/petshop.yaml", "#test", true, "_samples/petshop.yaml", Some("test"), Some("test"))]
  #[test_case("_samples/petshop.yaml", "test.json#test", false, "_samples/test.json", Some("test"), Some("test"))]
  #[test_case("_samples/petshop.yaml", "http://google.com/test.json#test", false, "http://google.com/test.json", Some("test"), Some("test"))]
  #[test_case("test.yaml", "test.yaml#/path", true, "test.yaml", Some("/path"), Some("path"))]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "#/definitions/Pet", true, "https://petstore.swagger.io/v2/swagger.json", Some("/definitions/Pet"), Some("Pet"))]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json#test", false, "http://google.com/test.json", Some("test"), Some("test"))]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json", false, "http://google.com/test.json", None, None)]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json", false, "https://petstore.swagger.io/test.json", None, None)]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json#fragment", false, "https://petstore.swagger.io/test.json", Some("fragment"), Some("fragment"))]
  fn refinfo_parse_tests(
    current_doc: &str,
    ref_path: &str,
    expected_is_nested: bool,
    expected_document_path: &str,
    expected_path: Option<&str>,
    expected_ref_friendly_name: Option<&str>,
  ) {
    let current_doc = DocumentPath::parse(current_doc).expect("?");
    let ref_info = RefInfo::parse(&current_doc, ref_path).expect("Should work");
    assert_eq!(ref_info.path, expected_path.map(|s| s.to_string()));
    assert_eq!(ref_info.is_nested, expected_is_nested);
    assert_eq!(ref_info.document_path, DocumentPath::parse(expected_document_path).expect("?"));
    assert_eq!(ref_info.ref_friendly_name, expected_ref_friendly_name.map(|s| s.to_string()));
  }

  #[test]
  fn reference_with_more_than_1_sharp_should_fail() {
    let failed = RefInfo::parse(&DocumentPath::None, "you.shall#not#path");
    let err = failed.expect_err("Should be an error");
    assert_eq!(
      err.to_string(),
      "There should be no more than 2 parts separated by # in a reference path."
    );
  }
}
