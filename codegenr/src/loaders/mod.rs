use serde_json::Value;
use thiserror::Error;
use url::Url;

pub mod document_path;
pub use document_path::*;
pub mod graphql;
pub mod json;
pub mod toml;
pub mod xml;
pub mod yaml;

pub trait DocumentLoader {
  type Error;
  fn json_from_str(content: &str) -> Result<Value, Self::Error>;
}

#[derive(Error, Debug)]
pub enum LoaderError {
  //
  // Io
  //
  #[error("Io Error: `{0}`.")]
  Io(#[from] std::io::Error),
  #[error("Can't read file `{0}`: `{1}`.")]
  Read(String, std::io::Error),
  #[error("Can't download file `{0}`: `{1}`.")]
  DownloadError(String, reqwest::Error),
  //
  // Path manipulation
  //
  #[error("Url `{url}`cannot be a base.")]
  UrlCannotBeABase { url: Url },
  #[error("Unable to append path `{from}` to `{to}`.")]
  UnableToAppendPath { to: String, from: String },
  #[error("The origin path should be a file and have parent.")]
  OriginPathShouldBeAFile { path: String },
  //
  // Deserialisation
  //
  #[error("Couldn't transpile yaml to json : `{0}`.")]
  YamlToJsonError(&'static str),
  #[error("Couldn't transpile graphql to json : `{0}`.")]
  GraphqlToJsonError(&'static str),
  #[error(
    "Could not read file content as json:\n-json_error: `{json_error}`\n-yaml_error:`{yaml_error}`\n-graphql_error:`{graphql_error}`."
  )]
  DeserialisationError {
    json_error: serde_json::Error,
    yaml_error: serde_yaml::Error,
    toml_error: ::toml::de::Error,
    xml_error: minidom::Error,
    graphql_error: graphql_parser::schema::ParseError,
  },
  #[error("Yaml error: `{0}`.")]
  YamlError(#[from] serde_yaml::Error),
  #[error("Json error: `{0}`.")]
  JsonError(#[from] serde_json::Error),
  #[error("Xml error: `{0}`.")]
  XmlError(#[from] minidom::Error),
  #[error("Graphql error: `{0}`.")]
  GraphqlError(#[from] graphql_parser::schema::ParseError),
  #[error("Did not try all the file loaders.")]
  DidNotTryAllFormats,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub(crate) enum FormatHint {
  /// The content should be json
  Json,
  /// The content should be yaml
  Yaml,
  /// The content should be toml
  Toml,
  /// The content should be xml
  Xml,
  /// The content should be a graphql schema
  Graphql,
  /// We have no f.....g idea
  NoIdea,
}

#[allow(clippy::result_large_err)]
fn json_from_string(content: &str, hint: FormatHint) -> Result<Value, LoaderError> {
  use FormatHint::*;
  match hint {
    FormatHint::Json | FormatHint::NoIdea => try_loaders(content, &[Json, Yaml, Toml, Graphql]),
    FormatHint::Yaml => try_loaders(content, &[Yaml, Json, Toml, Xml, Graphql]),
    FormatHint::Toml => try_loaders(content, &[Toml, Json, Yaml, Xml, Graphql]),
    FormatHint::Xml => try_loaders(content, &[Xml, Json, Yaml, Toml, Graphql]),
    FormatHint::Graphql => try_loaders(content, &[Graphql, Json, Yaml, Toml, Xml]),
  }
}

#[allow(clippy::result_large_err)]
fn try_loaders(content: &str, formats: &[FormatHint]) -> Result<Value, LoaderError> {
  let mut json_error: Option<serde_json::Error> = None;
  let mut yaml_error: Option<serde_yaml::Error> = None;
  let mut toml_error: Option<::toml::de::Error> = None;
  let mut xml_error: Option<::minidom::Error> = None;
  let mut graphql_error: Option<graphql_parser::schema::ParseError> = None;

  for hint in formats {
    match *hint {
      FormatHint::Json => {
        json_error = Some(match json::JsonLoader::json_from_str(content) {
          Ok(json) => return Ok(json),
          Err(e) => e,
        });
      }
      FormatHint::Yaml => {
        yaml_error = Some(match yaml::YamlLoader::json_from_str(content) {
          Ok(json) => return Ok(json),
          Err(e) => e,
        });
      }
      FormatHint::Toml => {
        toml_error = Some(match toml::TomlLoader::json_from_str(content) {
          Ok(json) => return Ok(json),
          Err(e) => e,
        });
      }
      FormatHint::Xml => {
        xml_error = Some(match xml::XmlLoader::json_from_str(content) {
          Ok(json) => return Ok(json),
          Err(e) => e,
        });
      }
      FormatHint::Graphql => {
        graphql_error = Some(match graphql::GraphqlLoader::json_from_str(content) {
          Ok(json) => return Ok(json),
          Err(e) => match e {
            LoaderError::GraphqlError(e) => e,
            _ => return Err(e), // this one should not happen
          },
        })
      }
      FormatHint::NoIdea => todo!(),
    }
  }

  Err(LoaderError::DeserialisationError {
    json_error: json_error.ok_or(LoaderError::DidNotTryAllFormats)?,
    yaml_error: yaml_error.ok_or(LoaderError::DidNotTryAllFormats)?,
    toml_error: toml_error.ok_or(LoaderError::DidNotTryAllFormats)?,
    xml_error: xml_error.ok_or(LoaderError::DidNotTryAllFormats)?,
    graphql_error: graphql_error.ok_or(LoaderError::DidNotTryAllFormats)?,
  })
}

#[cfg(test)]
mod tests {
  use crate::loaders::document_path::DocumentPath;

  use super::*;
  use test_case::test_case;

  #[test_case("h://f", "h://f", "h://f", "h://f")]
  #[test_case("h://w.com/api.yaml", "components.yaml", "h://w.com/components.yaml", "h://w.com/components.yaml")]
  #[test_case(
    "h://w.com/v1/api.yaml",
    "../v2/components.yaml",
    "h://w.com/v2/components.yaml",
    "h://w.com/\\v2\\components.yaml"
  )]
  #[test_case("file.yaml", "other.json", "other.json", "other.json")]
  #[test_case("test/file.yaml", "other.json", "test/other.json", "test\\other.json")]
  #[test_case("test/file.yaml", "./other2.json", "test/other2.json", "test\\other2.json")]
  #[test_case("test/file.yaml", "../other3.json", "other3.json", "other3.json")]
  #[test_case("test/file.yaml", "plop/other.json", "test/plop/other.json", "test\\plop/other.json")]
  #[test_case("file.yaml", "http://w.com/other.json", "http://w.com/other.json", "http://w.com/other.json")]
  #[test_case("file.json", "", "file.json", "file.json")]
  #[test_case("", "f", "f", "f")]
  #[test_case("", "h://f", "h://f", "h://f")]
  #[test_case(
    "_samples/petshop_with_external.yaml",
    "petshop_externals.yaml",
    "_samples/petshop_externals.yaml",
    "_samples\\petshop_externals.yaml"
  )]
  fn relate_test(doc_path: &str, ref_path: &str, expected_related: &str, win_expected: &str) {
    let doc_path = DocumentPath::parse(doc_path).expect("?");
    let r_path = DocumentPath::parse(ref_path).expect("?");
    let related = r_path.relate_from(&doc_path).expect("?");
    if cfg!(windows) {
      let expected_related = DocumentPath::parse(win_expected).expect("?");
      assert_eq!(related, expected_related);
    } else {
      let expected_related = DocumentPath::parse(expected_related).expect("?");
      assert_eq!(related, expected_related);
    }
  }

  #[allow(clippy::result_large_err)]
  #[test]
  fn read_json_file_test() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("./_samples/resolver/Merge1_rest.json")?.load_raw()?;
    Ok(())
  }

  #[allow(clippy::result_large_err)]
  #[test]
  fn read_yaml_file_test() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("./_samples/resolver/Merge1.yaml")?.load_raw()?;
    Ok(())
  }

  #[allow(clippy::result_large_err)]
  #[test]
  fn read_graph_file_test() -> Result<(), LoaderError> {
    let result = DocumentPath::parse("./_samples/graphql/schema.graphql")?.load_raw()?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
  }

  #[allow(clippy::result_large_err)]
  #[test]
  fn read_xml_file_test() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("./_samples/resolver/plant_catalog.xml")?.load_raw()?;
    dbg!(_result);
    Ok(())
  }

  #[allow(clippy::result_large_err)]
  #[test]
  #[ignore]
  fn read_beezup_openapi() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("https://api-docs.beezup.com/swagger.json")?.load_raw()?;
    Ok(())
  }
}
