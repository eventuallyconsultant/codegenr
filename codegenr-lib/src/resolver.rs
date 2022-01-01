use crate::{
  loader::{DocumentPath, LoaderError},
  OriginalDocumentsHash, ResolvedDocumentsHash,
};
use serde_json::Value;
use std::rc::Rc;
use thiserror::Error;

const REF: &str = "$ref";
const PATH_SEP: char = '/';
const SHARP_SEP: char = '#';
const FROM_REF: &str = "x-fromRef";
const REF_NAME: &str = "x-refName";

#[derive(Error, Debug)]
pub enum ResolverError {
  #[error("Loading error: `{0}`.")]
  Loading(#[from] LoaderError),
  #[error("Resolved `$ref` should be an Object to be merged to the importing object.")]
  ShouldBeObject,
  #[error("`$ref` value should be a String.")]
  ShouldBeString,
  #[error("Key `{key}` was not found in json part `{part2}`.")]
  KeyNotFound { key: String, part2: Value },
  #[error("Could not follow path `{0}` as json part is not an object.")]
  NotAnObject(String),
  #[error("`$ref` value `{0}` parse error. There should be no more than 2 parts separated by # in a reference path.")]
  NoMoreThanTwoParts(String),
}

enum Json {
  Original(Rc<Value>),
  Resolved(Rc<Value>),
}

fn ensure_orignal_json(doc_path: &DocumentPath, original_cache: &mut OriginalDocumentsHash) -> Result<Rc<Value>, ResolverError> {
  use std::collections::hash_map::Entry::*;
  match original_cache.entry(doc_path.clone()) {
    Occupied(entry) => Ok(entry.get().clone()),
    Vacant(entry) => {
      let json = doc_path.load_raw()?;
      let rc = Rc::new(json);
      entry.insert(rc.clone());
      Ok(rc)
    }
  }
}

fn get_resolved_json(doc_path: &DocumentPath, resolved_cache: &mut ResolvedDocumentsHash) -> Option<Rc<Value>> {
  resolved_cache.get(doc_path).cloned()
}

fn get_resolved_or_original(
  doc_path: &DocumentPath,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<Json, ResolverError> {
  match get_resolved_json(doc_path, resolved_cache) {
    Some(json) => Ok(Json::Resolved(json)),
    None => ensure_orignal_json(doc_path, original_cache).map(Json::Original),
  }
}

#[::tracing::instrument(level = "trace")]
pub fn resolve_refs_raw(json: Value) -> Result<Value, ResolverError> {
  let mut resolving = json.clone();
  resolve_refs_recurse(
    &DocumentPath::None,
    &mut resolving,
    &json,
    &mut Default::default(),
    &mut Default::default(),
  )?;
  Ok(resolving)
}

#[::tracing::instrument(level = "trace")]
pub fn resolve_refs(
  document: DocumentPath,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<Rc<Value>, ResolverError> {
  let json = get_resolved_or_original(&document, original_cache, resolved_cache)?;
  match json {
    Json::Resolved(j) => Ok(j),
    Json::Original(j) => {
      let mut resolving = (*j).clone();
      resolve_refs_recurse(&document, &mut resolving, &j, original_cache, resolved_cache)?;
      let resolved = Rc::new(resolving);
      resolved_cache.insert(document, resolved.clone());
      Ok(resolved)
    }
  }
}

fn resolve_refs_recurse(
  current_doc: &DocumentPath,
  json: &mut Value,
  original: &Value,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<(), ResolverError> {
  match json {
    Value::Array(a) => {
      for v in a {
        resolve_refs_recurse(current_doc, v, original, original_cache, resolved_cache)?;
      }
      Ok(())
    }
    Value::Object(obj) => {
      match obj.remove(REF) {
        Some(Value::String(ref_value)) => {
          let ref_info = RefInfo::parse(current_doc, &ref_value)?;

          let is_nested = ref_info.document_path == *current_doc;

          let new_value = if is_nested {
            let mut v = fetch_reference_value(original, &ref_info.path)?;
            resolve_refs_recurse(current_doc, &mut v, original, original_cache, resolved_cache)?;
            v
          } else {
            let doc_path = ref_info.document_path;
            let json = get_resolved_or_original(&doc_path, original_cache, resolved_cache)?;
            match json {
              Json::Resolved(j) => fetch_reference_value(&j, &ref_info.path)?,
              Json::Original(j) => {
                let mut v = fetch_reference_value(&j, &ref_info.path)?;
                resolve_refs_recurse(&doc_path, &mut v, &j, original_cache, resolved_cache)?;
                v
              }
            }
          };

          if let Value::Object(m) = new_value {
            for (k, v) in m {
              obj.insert(k, v);
            }
            obj.insert(FROM_REF.into(), Value::String(ref_value));
            obj.insert(
              REF_NAME.into(),
              Value::String(ref_info.path.map(|p| get_ref_name(&p)).unwrap_or_default()),
            );
          } else {
            return Err(ResolverError::ShouldBeObject);
          }
        }
        Some(_) => return Err(ResolverError::ShouldBeString),
        None => {}
      }

      for (_key, value) in obj.into_iter() {
        resolve_refs_recurse(current_doc, value, original, original_cache, resolved_cache)?;
      }
      Ok(())
    }
    _ => Ok(()),
  }
}

fn get_ref_name(path: &str) -> String {
  path.split(PATH_SEP).last().unwrap_or_default().to_string()
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
}

impl RefInfo {
  pub fn parse(doc_path: &DocumentPath, ref_value: &str) -> Result<Self, ResolverError> {
    let mut parts = ref_value.split(SHARP_SEP);

    let (ref_doc_path, path) = match (parts.next(), parts.next(), parts.next()) {
      (_, _, Some(_)) => return Err(ResolverError::NoMoreThanTwoParts(ref_value.into())),
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
    let json = resolve_refs(document, &mut Default::default(), &mut Default::default())?;
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
      "`$ref` value `you.shall#not#path` parse error. There should be no more than 2 parts separated by # in a reference path."
    );
  }

  #[test]
  fn very_tricky_test() -> Result<(), anyhow::Error> {
    let document = DocumentPath::parse("_samples/resolver/simple1.yaml")?;
    let json = resolve_refs(document, &mut Default::default(), &mut Default::default())?;
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
    assert_eq!(*json, expected);

    Ok(())
  }
}
