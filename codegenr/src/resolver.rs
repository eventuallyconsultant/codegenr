use serde_json::{Map, Value};
use url::Url;

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
    Value::Array(a) => {
      let mut new = Vec::<_>::with_capacity(a.len());
      for v in a {
        new.push(load_refs_recurse(v, original)?);
      }
      Ok(Value::Array(new))
    }
    Value::Object(obj) => {
      let mut map = Map::new();
      for (key, value) in obj.into_iter() {
        if key != REF {
          map.insert(key, load_refs_recurse(value, original)?);
        } else if let Value::String(path) = value {
          let new = resolve_reference(original, &path)?;
          match new {
            Value::Object(m) => {
              for (k, v) in m {
                map.insert(k, load_refs_recurse(v, original)?);
              }
              map.insert(FROM_REF.into(), Value::String(path.clone()));
              map.insert(REF_NAME.into(), Value::String(get_ref_name(&path)));
            }
            v => return load_refs_recurse(v, original),
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
        "x-refName": "myref",
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
  fn should_resolve_nested_references() -> Result<(), anyhow::Error> {
    let json = read_yaml_file("./_samples/petshop.yaml")?;
    let json = load_refs(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[test]
  fn should_resolve_external_references() -> Result<(), anyhow::Error> {
    let json = read_yaml_file("./_samples/petshop_with_external.yaml")?;
    let json = load_refs(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[rustfmt::skip]
  #[test_case("", "", true, true, "", None)]
  // #[test_case("_samples/petshop.yaml", "../test.json", false, true, "test.json", None)]
  // #[test_case("_samples/petshop.yaml", "test.json", false, true, "_samples/test.json", None)]
  // #[test_case("_samples/petshop.yaml", "#test", true, true, "_samples/petshop.yaml", Some("test"))]
  // #[test_case("_samples/petshop.yaml", "test.json#test", false, true, "_samples/test.json", Some("test"))]
  // #[test_case("_samples/petshop.yaml", "http://google.com/test.json#test", false, false, "http://google.com/test.json", Some("test"))]
  // #[test_case("test.yaml", "test.yaml#/path", true, true, "test.yaml", Some("/path"))]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "#/definitions/Pet", true, false, "https://petstore.swagger.io/v2/swagger.json", Some("/definitions/Pet"))]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json#test", false, false, "http://google.com/test.json", Some("test"))]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json", false, false, "http://google.com/test.json", None)]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json", false, false, "https://petstore.swagger.io/test.json", None)]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json#fragment", false, false, "https://petstore.swagger.io/test.json", Some("fragment"))]
  fn multiplication_tests(
    current_doc: &str,
    ref_path: &str,
    expected_is_nested: bool,
    _expected_is_local: bool,
    _expected_uri: &str,
    expected_path: Option<&str>,
  ) {
    let ref_info = RefInfo::parse(current_doc, ref_path).expect("Should work");
    assert_eq!(ref_info.path, expected_path.map(|s| s.to_string()));
    assert_eq!(ref_info.is_nested, expected_is_nested);
  }

  #[test]
  fn reference_with_more_than_1_sharp_should_fail() {
    let failed = RefInfo::parse("", "you.shall#not#path");
    let err = failed.expect_err("Should be an error");
    assert_eq!(err.to_string(), "Should be no more than 2 parts separated by # in a reference path");
  }

  #[derive(Debug)]
  struct RefInfo {
    /// Path of the reference to import in the destination file
    pub path: Option<String>,
    /// True if the reference is nested in the same document
    pub is_nested: bool,
    // pub is_local: bool,
    // pub abs_doc_uri: Url,

    // pub is_false_abs_ref: bool,
    // pub ref_friendly_name: String
    // public Uri AbsoluteDocumentUri { get; }
  }

  impl RefInfo {
    pub fn parse(doc_path: &str, ref_path: &str) -> Result<Self, anyhow::Error> {
      let mut path = None;
      let mut is_nested: bool = false;
      let mut parts = ref_path.split('#');

      match (parts.next(), parts.next(), parts.next()) {
        (_, _, Some(_)) => return Err(anyhow::anyhow!("Should be no more than 2 parts separated by # in a reference path")),
        (Some(file), None, None) => {
          is_nested = doc_path == file;
        }
        (Some(""), Some(p), None) => {
          is_nested = true;
          path = Some(p.to_string());
        }
        (Some(file), Some(p), None) => {
          is_nested = doc_path == file;
          path = Some(p.to_string());
        }
        (None, _, _) => todo!("nonono"),
      };

      Ok(Self { path, is_nested })
    }
  }
}

/*
  [Theory]

        public void GetRefInfo(string document, string @ref, bool expectedIsNested, bool expectedIsLocal, string expectedUri, string expectedPath)
        {
            var refInfo = RefInfo.GetRefInfo(document, @ref);

            if (!new Uri(expectedUri, UriKind.RelativeOrAbsolute).IsAbsoluteUri)
            {
                expectedUri = Path.Combine(Directory.GetCurrentDirectory(), expectedUri);
                expectedUri = new Uri(Path.GetFullPath(expectedUri)).AbsoluteUri.ToString();
            }
            else
            {
                expectedUri = new Uri(expectedUri).AbsoluteUri.ToString();
            }

            var absoluteUri = refInfo.AbsoluteDocumentUri.AbsoluteUri.ToString();

            refInfo.IsNestedInThisDocument.ShouldBe(expectedIsNested);
            refInfo.IsLocal.ShouldBe(expectedIsLocal);
            absoluteUri.ShouldBe(expectedUri);
            refInfo.InDocumentPath.ShouldBe(expectedPath);
        }
*/
