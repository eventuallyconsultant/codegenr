use crate::loader::{DocumentPath, LoaderError};
use serde_json::{Map, Value};
use std::collections::HashMap;
use thiserror::Error;

const REF: &str = "$ref";
const PATH_SEP: char = '/';
const SHARP_SEP: char = '#';
const FROM_REF: &str = "x-fromRef";
const REF_NAME: &str = "x-refName";

type DocumentsHash = HashMap<DocumentPath, Value>;

#[derive(Error, Debug)]
pub enum ResolverError {
  #[error("Loading errror: `{0}`.")]
  Loading(#[from] LoaderError),
  #[error("`{0}` value should be a String.")]
  ShouldBeString(&'static str),
  #[error("Key `{key}` was not found in json part `{part2}`.")]
  KeyNotFound { key: String, part2: Value },
  #[error("Could not follow path `{0}` as json part is not an object.")]
  NotAnObject(String),
  #[error("RefInfo parse error: `{0}`.")]
  NoMoreThanTwoParts(&'static str),
}

pub struct RefResolver {
  _hash: DocumentsHash,
}

impl RefResolver {
  fn new() -> Self {
    Self { _hash: Default::default() }
  }

  // fn jump<'a>(&'a mut self, parent_document_path: DocumentPath, parent_json: Value) -> RefResolverJump<'a> {
  //   self.hash.insert(parent_document_path.clone(), parent_json);
  //   RefResolverJump {
  //     ref_resolver: self,
  //     parent_document_path,
  //   }
  // }
}

// pub struct RefResolverJump<'a> {
//   pub ref_resolver: &'a RefResolver,
//   pub parent_document_path: DocumentPath,
// }

// impl<'a> Drop for RefResolverJump<'a> {
//   fn drop(&mut self) {}
// }

// #[cfg(test)]
// mod test2 {
//   use super::*;
//   #[test]
//   fn test() {
//     let mut rr = RefResolver::new();
//     let jump = rr.jump(DocumentPath::None, Value::Null);
//     drop(jump);
//     let _jump = rr.jump(DocumentPath::None, Value::Null);
//   }
// }

// impl RefResolver {
//   pub fn resolve_from_value(json: Value) -> Result<Value, anyhow::Error> {
//     todo!()
//   }

//   pub fn resolve_document(document_path: &str) -> Result<Value, anyhow::Error> {
//     todo!()
//   }
// }

// https://github.com/BeezUP/dotnet-codegen/tree/master/tests/CodegenUP.DocumentRefLoader.Tests

pub fn resolve_refs_raw(json: Value) -> Result<Value, ResolverError> {
  resolve_refs_recurse(&DocumentPath::None, json.clone(), &json, &mut Default::default())
}

pub fn resolve_refs(document: DocumentPath) -> Result<Value, ResolverError> {
  let json = document.load_raw()?;
  resolve_refs_recurse(&document, json.clone(), &json, &mut Default::default())
}

fn resolve_refs_recurse(
  current_doc: &DocumentPath,
  json: Value,
  original: &Value,
  cache: &mut DocumentsHash,
) -> Result<Value, ResolverError> {
  match json {
    Value::Array(a) => {
      let mut new = Vec::<_>::with_capacity(a.len());
      for v in a {
        new.push(resolve_refs_recurse(current_doc, v, original, cache)?);
      }
      Ok(Value::Array(new))
    }
    Value::Object(obj) => {
      let mut map = Map::new();
      for (key, value) in obj.into_iter() {
        if key != REF {
          map.insert(key, resolve_refs_recurse(current_doc, value, original, cache)?);
        } else if let Value::String(ref_value) = value {
          let ref_info = RefInfo::parse(current_doc, &ref_value)?;

          let is_nested = ref_info.document_path == *current_doc;

          let new_value = if is_nested {
            let v = fetch_reference_value(original, &ref_info.path)?;
            resolve_refs_recurse(current_doc, v, original, cache)?
          } else {
            let doc_path = ref_info.document_path;
            let json = load_raw_json(&doc_path, cache)?;
            // let json = cache.entry(doc_path).or_insert_with_key(|key| doc_path.load_raw()?);
            // let json = doc_path.load_raw()?;
            let v = fetch_reference_value(&json, &ref_info.path)?;
            resolve_refs_recurse(&doc_path, v, &json, cache)?
          };

          match new_value {
            Value::Object(m) => {
              for (k, v) in m {
                map.insert(k, v);
              }
              map.insert(FROM_REF.into(), Value::String(ref_value.clone()));
              map.insert(
                REF_NAME.into(),
                Value::String(ref_info.path.map(|p| get_ref_name(&p)).unwrap_or_default()),
              );
            }
            v => return Ok(v),
          }
        } else {
          return Err(ResolverError::ShouldBeString(REF));
        }
      }
      Ok(Value::Object(map))
    }
    _ => Ok(json),
  }
}

fn get_ref_name(path: &str) -> String {
  path.split(PATH_SEP).last().unwrap_or_default().to_string()
}

fn load_raw_json(doc_path: &DocumentPath, cache: &mut DocumentsHash) -> Result<Value, ResolverError> {
  let getter = cache.get(doc_path);
  if let Some(json) = getter {
    return Ok(json.clone());
  }

  let json = doc_path.load_raw()?;
  cache.insert(doc_path.clone(), json);
  Ok(
    cache
      .get(doc_path)
      .expect("Just inserrted the value. For sure its existing!")
      .clone(),
  )
  // match cache.get(&doc_path) {
  //   Some(json) => Ok(json),
  //   None => {
  //     let json = doc_path.load_raw()?;
  //     cache.insert(doc_path.clone(), json);
  //     Ok(cache.get(&doc_path).expect("Just inserrted the value. For sure its existing!"))
  //   }
  // }
}

fn fetch_reference_value(json: &Value, path: &Option<String>) -> Result<Value, ResolverError> {
  match path {
    Some(p) => {
      let parts = p.split(PATH_SEP);
      let mut part = json;
      for p in parts.filter(|p| !p.trim().is_empty()) {
        if let Value::Object(o) = part {
          let key = p.to_string();
          let part2 = part.clone();
          part = o.get(p).ok_or(ResolverError::KeyNotFound { key, part2 })?;
        } else {
          let key = p.to_string();
          return Err(ResolverError::NotAnObject(key));
        }
      }
      Ok(part.clone())
    }
    None => Ok(json.clone()),
  }
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
  pub fn parse(doc_path: &DocumentPath, ref_value: &str) -> Result<Self, ResolverError> {
    let mut parts = ref_value.split(SHARP_SEP);

    let (ref_doc_path, path) = match (parts.next(), parts.next(), parts.next()) {
      (_, _, Some(_)) => {
        return Err(ResolverError::NoMoreThanTwoParts(
          "There should be no more than 2 parts separated by # in a reference path.",
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
      fetch_reference_value(&json, &Some("/test/data1/value".into()))?,
      Value::Number(serde_json::Number::from(42))
    );

    assert_eq!(fetch_reference_value(&json, &Some("/test/data1".into()))?, json!({ "value": 42 }));

    let path: &str = "/test/not_existing_path";
    let failed_test = fetch_reference_value(&json, &Some(path.into()));
    let err = failed_test.expect_err("Should be an error");
    assert_eq!(
      err.to_string(),
      "Key `not_existing_path` was not found in json part `{\"data1\":{\"value\":42},\"data2\":[1,2,3]}`."
    );

    Ok(())
  }

  #[test]
  fn resolve_refs_test_0() -> Result<(), anyhow::Error> {
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
  fn resolve_refs_test_1() -> Result<(), anyhow::Error> {
    let json = json!({
      "test": {
        "$ref": "#myref"
      },
      "myref": {
        "data": "test"
      }
    });

    let expected = json!({
      "test": {
        "data": "test",
        "x-fromRef": "#myref",
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
    let json = DocumentPath::parse("_samples/resolver/petshop.yaml")?.load_raw()?;
    let json = resolve_refs_raw(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[test]
  fn should_resolve_external_references() -> Result<(), anyhow::Error> {
    let document = DocumentPath::parse("_samples/resolver/petshop_with_external.yaml")?;
    let json = resolve_refs(document)?;
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
    
    let expected_document_path =
    if cfg!(windows) && !expected_is_nested {
      DocumentPath::parse(expected_document_path.replace("/","\\").as_str()).expect("?")
    } else {
      DocumentPath::parse(expected_document_path).expect("?")
    };
    assert_eq!(ref_info.document_path, expected_document_path);
    
    assert_eq!(ref_info.ref_friendly_name, expected_ref_friendly_name.map(|s| s.to_string()));
  }

  #[test]
  fn reference_with_more_than_1_sharp_should_fail() {
    let failed = RefInfo::parse(&DocumentPath::None, "you.shall#not#path");
    let err = failed.expect_err("Should be an error");
    assert_eq!(
      err.to_string(),
      "RefInfo parse error: `There should be no more than 2 parts separated by # in a reference path.`."
    );
  }

  #[test]
  fn very_tricky_test() -> Result<(), anyhow::Error> {
    let document = DocumentPath::parse("_samples/resolver/simple1.yaml")?;
    let json = resolve_refs(document)?;
    let string = json.to_string();
    assert!(!string.contains(REF));

    let expected = json!({
      "test": {
        "this": "will load multiple files"
      },
      "finalvalue": {
        "value": "this is the real final value"
      },
      "value": {
        "subvalue": {
          "value": "this is the real final value",
          "x-fromRef": "simple3.yaml#/subSubValue/value",
          "x-refName": "value"
        },
        "x-fromRef": "simple2.json",
        "x-refName": ""
      }
    });
    assert_eq!(json, expected);

    Ok(())
  }
}
