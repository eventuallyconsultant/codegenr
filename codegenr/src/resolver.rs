use path_dedot::*;
use relative_path::RelativePath;
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

#[derive(Debug)]
pub struct RefInfo {
  /// Path of the reference to import in the destination file
  pub path: Option<String>,
  /// True if the reference is nested in the same document
  pub is_nested: bool,
  /// File path of the document containing the reference
  pub document_path: DocumentPath,
  // pub abs_doc_uri: Url,

  // pub is_false_abs_ref: bool,
  // pub ref_friendly_name: String
  // public Uri AbsoluteDocumentUri { get; }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DocumentPath {
  /// Full url to a file : https://mywebsite/api.yaml
  Url(Url),
  /// File name or relative file name
  FileName(String),
  None,
}

impl DocumentPath {
  pub fn parse(ref_path: &str) -> Result<Self, anyhow::Error> {
    Ok(if ref_path.trim() == "" {
      Self::None
    } else {
      match Url::parse(ref_path) {
        Ok(url) => DocumentPath::Url(url),
        Err(_) => DocumentPath::FileName(ref_path.into()),
      }
    })
  }

  pub fn relate_from(self, refed_from: &Self) -> Result<Self, anyhow::Error> {
    use DocumentPath::*;
    Ok(match (refed_from, self) {
      (Url(_url_from), Url(_url_to)) => todo!(),
      (Url(_url_from), FileName(_path_to)) => todo!(),
      (Url(_url_from), None) => refed_from.clone(),
      (FileName(_path_from), FileName(_path_to)) => todo!(),
      (FileName(_path_from), Url(_url_to)) => todo!(),
      (FileName(_path_from), None) => refed_from.clone(),
      (None, s) => s,
    })
  }
}

impl RefInfo {
  pub fn parse(doc_path: &str, ref_value: &str) -> Result<Self, anyhow::Error> {
    let mut path = None;
    let mut is_nested: bool = false;
    let mut parts = ref_value.split('#');
    let mut target_document_path = doc_path.to_string();

    match (parts.next(), parts.next(), parts.next()) {
      (_, _, Some(_)) => {
        return Err(anyhow::anyhow!(
          "There should be no more than 2 parts separated by # in a reference path."
        ))
      }
      (Some(file), None, None) => {
        target_document_path = file.to_string();
        is_nested = doc_path == file;
      }
      (Some(""), Some(p), None) => {
        is_nested = true;
        path = Some(p.to_string());
      }
      (Some(file), Some(p), None) => {
        target_document_path = file.to_string();
        is_nested = doc_path == file;
        path = Some(p.to_string());
      }
      (None, _, _) => unreachable!("Split always returns at least one element"),
    };

    let document_path = match Url::parse(&target_document_path) {
      Ok(url) => DocumentPath::Url(url),
      Err(_) => {
        let dir = RelativePath::new(doc_path)
          .parent()
          .ok_or_else(|| anyhow::anyhow!("Should have a parent."))?;
        let p = dir.join(target_document_path);
        let p = p.to_path(".");
        let x = p
          .parse_dot_from(std::path::Path::new("."))?
          .to_str()
          .ok_or_else(|| anyhow::anyhow!("Should have a parent."))?
          .to_string();
        DocumentPath::FileName(x)
      }
    };

    Ok(Self {
      path,
      is_nested,
      document_path,
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
  // #[test_case("", "", true, "", None)]
  #[test_case("_samples/petshop.yaml", "../test.json", false, DocumentPath::FileName("test.json".into()), None)]
  // #[test_case("_samples/petshop.yaml", "test.json", false, DocumentPath::Local("_samples/test.json".into()), None)]
  // #[test_case("_samples/petshop.yaml", "#test", true, DocumentPath::Local("_samples/petshop.yaml".into()), Some("test"))]
  // #[test_case("_samples/petshop.yaml", "test.json#test", false, DocumentPath::Local("_samples/test.json".into()), Some("test"))]
  // #[test_case("_samples/petshop.yaml", "http://google.com/test.json#test", false, DocumentPath::Url(Url::parse("http://google.com/test.json").expect("")), Some("test"))]
  // #[test_case("test.yaml", "test.yaml#/path", true, DocumentPath::Local("test.yaml".into()), Some("/path"))]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "#/definitions/Pet", true, DocumentPath::Url(Url::parse("https://petstore.swagger.io/v2/swagger.json").expect("")), Some("/definitions/Pet"))]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json#test", false, DocumentPath::Url(Url::parse("http://google.com/test.json").expect("")), Some("test"))]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json", false, DocumentPath::Url(Url::parse("http://google.com/test.json").expect("")), None)]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json", false, DocumentPath::Url(Url::parse("https://petstore.swagger.io/test.json").expect("")), None)]
  // #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json#fragment", false, DocumentPath::Url(Url::parse("https://petstore.swagger.io/test.json").expect("")), Some("fragment"))]
  fn refinfo_parse_tests(
    current_doc: &str,
    ref_path: &str,
    expected_is_nested: bool,
    expected_document_path: DocumentPath,
    expected_path: Option<&str>,
  ) {
    let ref_info = RefInfo::parse(current_doc, ref_path).expect("Should work");
    assert_eq!(ref_info.path, expected_path.map(|s| s.to_string()));
    assert_eq!(ref_info.is_nested, expected_is_nested);
    assert_eq!(ref_info.document_path, expected_document_path);
  }

  #[test]
  fn reference_with_more_than_1_sharp_should_fail() {
    let failed = RefInfo::parse("", "you.shall#not#path");
    let err = failed.expect_err("Should be an error");
    assert_eq!(
      err.to_string(),
      "There should be no more than 2 parts separated by # in a reference path."
    );
  }

  #[test_case(DocumentPath::Url(Url::parse("h://f").expect("?")), "h://f", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  // #[test_case(DocumentPath::Url(Url::parse("h://f").expect("?")), "", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  // #[test_case(DocumentPath::FileName("f".into()), "", DocumentPath::FileName("f".into()))]
  // #[test_case(DocumentPath::None, "f", DocumentPath::FileName("f".into()))]
  // #[test_case(DocumentPath::None, "h://f", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  fn relate_test(doc_path: DocumentPath, ref_path: &str, expected_related: DocumentPath) {
    let r_path = DocumentPath::parse(ref_path).expect("?");
    let related = r_path.relate_from(&doc_path).expect("?");
    assert_eq!(related, expected_related);
  }

  #[test]
  fn test() {
    // Url::
    // let url = Url::parse("pouet://test.com/test.html").expect("azegiouh");
    // assert_eq!(url.scheme(), "pouet");
    // assert!(url.cannot_be_a_base());

    use std::path::Path;
    Path::new("./foo/bar.txt");
    let p = Path::new("http://test.com/foo/bar.txt/../test");
    dbg!(p.parent());
    // let can = p.canonicalize().expect("");
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
