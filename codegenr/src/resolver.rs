use crate::{
  OriginalDocumentsHash, ResolvedDocumentsHash,
  loaders::{DocumentPath, LoaderError},
};
use path_dedot::ParseDot;
use serde_json::Value;
use std::{
  path::{Path, PathBuf},
  rc::Rc,
};
use thiserror::Error;
use url::Url;

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
  #[error("Key `{key}` was not found in json part `{part}`.")]
  KeyNotFound { key: String, part: Value },
  #[error("Could not follow path `{0}` as json part is not an object.")]
  NotAnObject(String),
  #[error("`$ref` value `{0}` parse error. There should be no more than 2 parts separated by # in a reference path.")]
  NoMoreThanTwoParts(String),
  #[error(
    "Downloading `{url}` needs your approval: add `{parent}`, or any url above it, to the `allowed_sources` of the section in `codegenr.toml` \
     (`allowed_sources = ['{parent}']`), or pass `--allowed-sources {parent}` to `gen`."
  )]
  UrlNotAllowed { url: String, parent: String },
  #[error(
    "Reading `{path}` needs your approval, it is outside the current folder and the folder of `source`: add `{parent}`, or any folder above it, \
     to the `allowed_sources` of the section in `codegenr.toml` (`allowed_sources = ['{parent}']`), or pass `--allowed-sources {parent}` to `gen`."
  )]
  FileNotAllowed { path: String, parent: String },
}

/// Where a section reads its documents from, the source and every file or url its `$ref` lead to: they are not trusted.
/// A file must be under the current folder, the folder of the section `source`, or a folder of `allowed_sources`;
/// an url must be under an url of `allowed_sources`: downloading is always the decision of the user.
// TO_HUMAN: the check is lexical, a symbolic link under an allowed folder can still lead out of it, its target is not checked.
#[derive(Debug)]
pub struct AllowedSources {
  current_folder: PathBuf,
  folders: Vec<PathBuf>,
  urls: Vec<Url>,
}

impl AllowedSources {
  #[allow(clippy::result_large_err)]
  pub fn new(source: &DocumentPath, allowed_sources: &[String]) -> Result<Self, ResolverError> {
    let current_folder = std::env::current_dir().map_err(LoaderError::from)?;
    let mut folders = vec![current_folder.clone()];
    if let DocumentPath::FileName(path) = source {
      folders.extend(absolute(&current_folder, path)?.parent().map(Path::to_path_buf));
    }
    let mut urls = Vec::new();
    for allowed in allowed_sources {
      match DocumentPath::parse(allowed)? {
        DocumentPath::Url(url) => urls.push(url),
        DocumentPath::FileName(folder) => folders.push(absolute(&current_folder, &folder)?),
        DocumentPath::None => {}
      }
    }
    Ok(Self {
      current_folder,
      folders,
      urls,
    })
  }

  #[allow(clippy::result_large_err)]
  fn check(&self, document: &DocumentPath) -> Result<(), ResolverError> {
    match document {
      DocumentPath::Url(url) if !self.urls.iter().any(|allowed| is_under_url(url, allowed)) => Err(ResolverError::UrlNotAllowed {
        url: url.to_string(),
        parent: url.join("./").map_or_else(|_| url.to_string(), |parent| parent.to_string()),
      }),
      DocumentPath::FileName(path) => {
        let path = absolute(&self.current_folder, path)?;
        match self.folders.iter().any(|folder| path.starts_with(folder)) {
          true => Ok(()),
          false => Err(ResolverError::FileNotAllowed {
            path: path.display().to_string(),
            parent: path.parent().unwrap_or(&path).display().to_string(),
          }),
        }
      }
      _ => Ok(()),
    }
  }
}

/// The path, absolute and without `.` nor `..`, worked out without reading the file system.
fn absolute(current_folder: &Path, path: &str) -> Result<PathBuf, LoaderError> {
  Ok(current_folder.join(path).parse_dot()?.into_owned())
}

/// Same scheme, host and port, and the path of `url` starts with the whole path of `allowed`.
fn is_under_url(url: &Url, allowed: &Url) -> bool {
  fn segments(url: &Url) -> Vec<&str> {
    url
      .path_segments()
      .map(|segments| segments.filter(|s| !s.is_empty()).collect())
      .unwrap_or_default()
  }
  url.scheme() == allowed.scheme()
    && url.host_str() == allowed.host_str()
    && url.port_or_known_default() == allowed.port_or_known_default()
    && segments(url).starts_with(&segments(allowed))
}

enum Json {
  Original(Rc<Value>),
  Resolved(Rc<Value>),
}

#[allow(clippy::result_large_err)]
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

#[allow(clippy::result_large_err)]
fn get_resolved_or_original(
  doc_path: &DocumentPath,
  allowed: &AllowedSources,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<Json, ResolverError> {
  // Before the caches: they are shared by the sections, which may not allow the same sources.
  allowed.check(doc_path)?;
  match get_resolved_json(doc_path, resolved_cache) {
    Some(json) => Ok(Json::Resolved(json)),
    None => ensure_orignal_json(doc_path, original_cache).map(Json::Original),
  }
}

#[allow(clippy::result_large_err)]
#[::tracing::instrument(level = "trace")]
pub fn resolve_refs_raw(json: Value) -> Result<Value, ResolverError> {
  let mut resolving = json.clone();
  let allowed = AllowedSources::new(&DocumentPath::None, &[])?;
  resolve_refs_recurse(
    &DocumentPath::None,
    &mut resolving,
    &json,
    &allowed,
    &mut Default::default(),
    &mut Default::default(),
  )?;
  Ok(resolving)
}

#[allow(clippy::result_large_err)]
#[::tracing::instrument(level = "trace")]
pub fn resolve_refs(
  document: DocumentPath,
  allowed: &AllowedSources,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<Rc<Value>, ResolverError> {
  let json = get_resolved_or_original(&document, allowed, original_cache, resolved_cache)?;
  match json {
    Json::Resolved(j) => Ok(j),
    Json::Original(j) => {
      let mut resolving = (*j).clone();
      resolve_refs_recurse(&document, &mut resolving, &j, allowed, original_cache, resolved_cache)?;
      let resolved = Rc::new(resolving);
      resolved_cache.insert(document, resolved.clone());
      Ok(resolved)
    }
  }
}

#[allow(clippy::result_large_err)]
fn resolve_refs_recurse(
  current_doc: &DocumentPath,
  json: &mut Value,
  original: &Value,
  allowed: &AllowedSources,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<(), ResolverError> {
  match json {
    Value::Array(a) => {
      for v in a {
        resolve_refs_recurse(current_doc, v, original, allowed, original_cache, resolved_cache)?;
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
            resolve_refs_recurse(current_doc, &mut v, original, allowed, original_cache, resolved_cache)?;
            v
          } else {
            let doc_path = ref_info.document_path;
            let json = get_resolved_or_original(&doc_path, allowed, original_cache, resolved_cache)?;
            match json {
              Json::Resolved(j) => fetch_reference_value(&j, &ref_info.path)?,
              Json::Original(j) => {
                let mut v = fetch_reference_value(&j, &ref_info.path)?;
                resolve_refs_recurse(&doc_path, &mut v, &j, allowed, original_cache, resolved_cache)?;
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
        resolve_refs_recurse(current_doc, value, original, allowed, original_cache, resolved_cache)?;
      }
      Ok(())
    }
    _ => Ok(()),
  }
}

fn get_ref_name(path: &str) -> String {
  path.split(PATH_SEP).next_back().unwrap_or_default().to_string()
}

#[allow(clippy::result_large_err)]
fn fetch_reference_value(json: &Value, path: &Option<String>) -> Result<Value, ResolverError> {
  match path {
    Some(p) => {
      let parts = p.split(PATH_SEP);
      let mut part = json;
      for p in parts.filter(|p| !p.trim().is_empty()) {
        if let Value::Object(o) = part {
          let key = p.to_string();
          part = o.get(p).ok_or_else(|| ResolverError::KeyNotFound { key, part: part.clone() })?;
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
  #[allow(clippy::result_large_err)]
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
    let ref_friendly_name = path.as_ref().map(|p| p.split('/').next_back().unwrap_or_default().to_string());

    Ok(Self {
      path,
      is_nested,
      document_path: ref_doc_path,
      ref_friendly_name,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::loaders::*;
  use serde_json::json;
  use std::io::Write;
  use tempfile::TempDir;
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
    println!("{}", resolved);
    println!("{}", expected);
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
    println!("{}", resolved);
    println!("{}", expected);
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
    println!("{}", resolved);
    println!("{}", expected);
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
    println!("{}", resolved);
    println!("{}", expected);
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
    println!("{}", resolved);
    println!("{}", expected);
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
    let allowed = AllowedSources::new(&document, &[])?;
    let json = resolve_refs(document, &allowed, &mut Default::default(), &mut Default::default())?;
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
      DocumentPath::parse(expected_document_path.replace('/',"\\").as_str()).expect("?")
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
    let allowed = AllowedSources::new(&document, &[])?;
    let json = resolve_refs(document, &allowed, &mut Default::default(), &mut Default::default())?;
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

  #[test_case("https://host/api/", "https://host/api/spec.yaml", true ; "file in the allowed folder")]
  #[test_case("https://host/api/", "https://host/api/v2/spec.yaml", true ; "file in a subfolder")]
  #[test_case("https://host/api/spec.yaml", "https://host/api/spec.yaml", true ; "the allowed file itself")]
  #[test_case("https://host", "https://host/any/spec.yaml", true ; "the whole host")]
  #[test_case("https://host:443/api/", "https://host/api/spec.yaml", true ; "default port written")]
  #[test_case("https://host/api/", "https://host/api-evil/spec.yaml", false ; "folder whose name starts the same")]
  #[test_case("https://host/api/", "https://host/spec.yaml", false ; "parent folder")]
  #[test_case("https://host/api/", "https://host/api/../secret.yaml", false ; "dot dot leaving the folder")]
  #[test_case("https://host/api/", "https://host/api/%2e%2e/secret.yaml", false ; "encoded dot dot leaving the folder")]
  #[test_case("https://host/api/", "http://host/api/spec.yaml", false ; "other scheme")]
  #[test_case("https://host/api/", "https://host:8443/api/spec.yaml", false ; "other port")]
  #[test_case("https://host/api/", "https://other.host/api/spec.yaml", false ; "other host")]
  fn an_url_is_allowed_only_under_an_allowed_url(allowed: &str, url: &str, expected: bool) -> anyhow::Result<()> {
    let allowed = AllowedSources::new(&DocumentPath::None, &[allowed.into()])?;
    assert_eq!(allowed.check(&DocumentPath::parse(url)?).is_ok(), expected);
    Ok(())
  }

  /// `.invalid` never resolves: had the check been skipped, the error would be a download one.
  #[test]
  fn an_url_not_allowed_is_not_downloaded() -> anyhow::Result<()> {
    let document = DocumentPath::parse("https://example.invalid/api/spec.yaml?v=2")?;
    let allowed = AllowedSources::new(&document, &[])?;

    let error = resolve_refs(document, &allowed, &mut Default::default(), &mut Default::default()).expect_err("No url is allowed.");

    assert_eq!(
      error.to_string(),
      "Downloading `https://example.invalid/api/spec.yaml?v=2` needs your approval: add `https://example.invalid/api/`, or any url above it, \
       to the `allowed_sources` of the section in `codegenr.toml` (`allowed_sources = ['https://example.invalid/api/']`), \
       or pass `--allowed-sources https://example.invalid/api/` to `gen`."
    );
    Ok(())
  }

  /// The temp folder is out of the current folder, the crate one for `cargo test`.
  fn write_file(root: &std::path::Path, path: &str, content: &str) -> anyhow::Result<()> {
    let (mut file, _) = crate::filesystem::create_file(root, path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
  }

  #[test_case(false ; "relative path climbing out")]
  #[test_case(true ; "absolute path")]
  fn a_ref_to_a_file_out_of_the_allowed_folders_is_refused(absolute_ref: bool) -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("RESOLVER_tests")?;
    let secret = tmp.path().join("secret");
    let reference = match absolute_ref {
      true => secret.join("token.yaml").display().to_string(),
      false => "../secret/token.yaml".into(),
    };
    write_file(tmp.path(), "specs/api.yaml", &format!("a:\n  $ref: '{reference}'\n"))?;
    write_file(tmp.path(), "secret/token.yaml", "token: abc\n")?;
    let document = DocumentPath::parse(&tmp.path().join("specs/api.yaml").display().to_string())?;

    let refusing = AllowedSources::new(&document, &[])?;
    let error = resolve_refs(document.clone(), &refusing, &mut Default::default(), &mut Default::default())
      .expect_err("The secret folder is not allowed.");
    // On Windows, the drive letter of an absolute path reads as the scheme of an url.
    assert!(
      matches!(error, ResolverError::FileNotAllowed { .. } | ResolverError::UrlNotAllowed { .. }),
      "{error}"
    );

    let allowing = AllowedSources::new(&document, &[secret.display().to_string()])?;
    if !(cfg!(windows) && absolute_ref) {
      let json = resolve_refs(document, &allowing, &mut Default::default(), &mut Default::default())?;
      assert_eq!(json["a"]["token"], json!("abc"));
    }
    Ok(())
  }
}
