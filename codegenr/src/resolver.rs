use path_dedot::*;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::Path;
use url::Url;

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
      (Url(_), Url(url)) => Url(url),
      (Url(url_from), FileName(path_to)) => {
        let mut url = url_from.clone();
        url.path_segments_mut().map_err(|_| anyhow::anyhow!("Url cannot be a base."))?.pop();
        let path = url.path();
        let new_path = Path::new(path).join(&path_to);
        let new_path = new_path.parse_dot()?;
        let new_path = new_path
          .to_str()
          .ok_or_else(|| anyhow::anyhow!("Unable to append path '{}' to '{}'", path_to, url_from))?;
        url.set_path(new_path);
        Url(url)
      }
      (Url(_), None) => refed_from.clone(),
      (FileName(path_from), FileName(path_to)) => {
        let folder = Path::new(path_from)
          .parent()
          .ok_or_else(|| anyhow::anyhow!("The origin path should be a file and have parent."))?;
        folder
          .join(&path_to)
          .parse_dot()?
          .to_str()
          .map(|s| FileName(s.to_owned()))
          .ok_or_else(|| anyhow::anyhow!("Unable to append path '{}' to '{}'", path_to, path_from))?
      }
      (FileName(_), Url(url)) => Url(url),
      (FileName(_path_from), None) => refed_from.clone(),
      (None, s) => s,
    })
  }
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

    Ok(Self {
      path,
      is_nested,
      document_path: ref_doc_path,
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
    let json = read_yaml_file("./_samples/petshop.yaml")?;
    let json = resolve_refs_raw(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[test]
  fn should_resolve_external_references() -> Result<(), anyhow::Error> {
    let json = read_yaml_file("./_samples/petshop_with_external.yaml")?;
    let json = resolve_refs_raw(json)?;
    let string = json.to_string();
    assert!(!string.contains(REF));
    Ok(())
  }

  #[rustfmt::skip]
  #[test_case("", "", true, DocumentPath::None, None)]
  #[test_case("_samples/petshop.yaml", "../test.json", false, DocumentPath::FileName("test.json".into()), None)]
  #[test_case("_samples/petshop.yaml", "test.json", false, DocumentPath::FileName("_samples/test.json".into()), None)]
  #[test_case("_samples/petshop.yaml", "#test", true, DocumentPath::FileName("_samples/petshop.yaml".into()), Some("test"))]
  #[test_case("_samples/petshop.yaml", "test.json#test", false, DocumentPath::FileName("_samples/test.json".into()), Some("test"))]
  #[test_case("_samples/petshop.yaml", "http://google.com/test.json#test", false, DocumentPath::Url(Url::parse("http://google.com/test.json").expect("")), Some("test"))]
  #[test_case("test.yaml", "test.yaml#/path", true, DocumentPath::FileName("test.yaml".into()), Some("/path"))]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "#/definitions/Pet", true, DocumentPath::Url(Url::parse("https://petstore.swagger.io/v2/swagger.json").expect("")), Some("/definitions/Pet"))]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json#test", false, DocumentPath::Url(Url::parse("http://google.com/test.json").expect("")), Some("test"))]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "http://google.com/test.json", false, DocumentPath::Url(Url::parse("http://google.com/test.json").expect("")), None)]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json", false, DocumentPath::Url(Url::parse("https://petstore.swagger.io/test.json").expect("")), None)]
  #[test_case("https://petstore.swagger.io/v2/swagger.json", "../test.json#fragment", false, DocumentPath::Url(Url::parse("https://petstore.swagger.io/test.json").expect("")), Some("fragment"))]
  fn refinfo_parse_tests(
    current_doc: &str,
    ref_path: &str,
    expected_is_nested: bool,
    expected_document_path: DocumentPath,
    expected_path: Option<&str>,
  ) {
    let current_doc = DocumentPath::parse(current_doc).expect("?");
    let ref_info = RefInfo::parse(&current_doc, ref_path).expect("Should work");
    assert_eq!(ref_info.path, expected_path.map(|s| s.to_string()));
    assert_eq!(ref_info.is_nested, expected_is_nested);
    assert_eq!(ref_info.document_path, expected_document_path);
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

  #[test_case(DocumentPath::Url(Url::parse("h://f").expect("?")), "h://f", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  #[test_case(DocumentPath::Url(Url::parse("h://w.com/api.yaml").expect("?")), "components.yaml", DocumentPath::Url(Url::parse("h://w.com/components.yaml").expect("?")))]
  #[test_case(DocumentPath::Url(Url::parse("h://w.com/v1/api.yaml").expect("?")), "../v2/components.yaml", DocumentPath::Url(Url::parse("h://w.com/v2/components.yaml").expect("?")))]
  #[test_case(DocumentPath::Url(Url::parse("h://f").expect("?")), "", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  #[test_case(DocumentPath::FileName("file.yaml".into()), "other.json", DocumentPath::FileName("other.json".into()))]
  #[test_case(DocumentPath::FileName("test/file.yaml".into()), "other.json", DocumentPath::FileName("test/other.json".into()))]
  #[test_case(DocumentPath::FileName("test/file.yaml".into()), "./other2.json", DocumentPath::FileName("test/other2.json".into()))]
  #[test_case(DocumentPath::FileName("test/file.yaml".into()), "../other3.json", DocumentPath::FileName("other3.json".into()))]
  #[test_case(DocumentPath::FileName("test/file.yaml".into()), "plop/other.json", DocumentPath::FileName("test/plop/other.json".into()))]
  #[test_case(DocumentPath::FileName("file.yaml".into()), "http://w.com/other.json", DocumentPath::Url(Url::parse("http://w.com/other.json").expect("?")))]
  #[test_case(DocumentPath::FileName("file.json".into()), "", DocumentPath::FileName("file.json".into()))]
  #[test_case(DocumentPath::None, "f", DocumentPath::FileName("f".into()))]
  #[test_case(DocumentPath::None, "h://f", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  fn relate_test(doc_path: DocumentPath, ref_path: &str, expected_related: DocumentPath) {
    let r_path = DocumentPath::parse(ref_path).expect("?");
    let related = r_path.relate_from(&doc_path).expect("?");
    assert_eq!(related, expected_related);
  }

  #[test_case("h://f", DocumentPath::Url(Url::parse("h://f").expect("?")))]
  #[test_case("file.json", DocumentPath::FileName("file.json".into()))]
  #[test_case("./file2.json", DocumentPath::FileName("./file2.json".into()))]
  #[test_case("../file3.json", DocumentPath::FileName("../file3.json".into()))]
  #[test_case(" ", DocumentPath::None)]
  fn path_parse(ref_path: &str, expected: DocumentPath) {
    let r_path = DocumentPath::parse(ref_path).expect("?");
    assert_eq!(r_path, expected);
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
