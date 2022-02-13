use serde_json::Value;
use thiserror::Error;
use url::Url;

pub mod document_path;
pub use document_path::*;
pub mod graphql;
pub mod json;
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
  #[error("Could not read file content as json:\n-json_error: `{json_error}`\n-yaml_error:`{yaml_error}`.")]
  DeserialisationError {
    json_error: serde_json::Error,
    yaml_error: serde_yaml::Error,
  },
  #[error("Yaml error: `{0}`.")]
  YamlError(#[from] serde_yaml::Error),
  #[error("Json error: `{0}`.")]
  JsonError(#[from] serde_json::Error),
  #[error("Graphql error: `{0}`.")]
  GraphqlError(#[from] graphql_parser::schema::ParseError),
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub(crate) enum FormatHint {
  /// The content should be json
  Json,
  /// The content should be yaml
  Yaml,
  /// The content should be a graphql schema
  // Graphql,
  /// We have no f.....g idea
  NoIdea,
}

fn json_from_string(content: &str, hint: FormatHint) -> Result<Value, LoaderError> {
  match hint {
    FormatHint::Json | FormatHint::NoIdea => {
      let json_error = match json::JsonLoader::json_from_str(content) {
        Ok(json) => return Ok(json),
        Err(e) => e,
      };
      let yaml_error = match yaml::YamlLoader::json_from_str(content) {
        Ok(yaml) => return Ok(yaml),
        Err(e) => e,
      };
      Err(LoaderError::DeserialisationError { json_error, yaml_error })
    }
    FormatHint::Yaml => {
      let yaml_error = match yaml::YamlLoader::json_from_str(content) {
        Ok(yaml) => return Ok(yaml),
        Err(e) => e,
      };
      let json_error = match json::JsonLoader::json_from_str(content) {
        Ok(json) => return Ok(json),
        Err(e) => e,
      };
      Err(LoaderError::DeserialisationError { json_error, yaml_error })
    }
  }
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

  #[test]
  fn read_json_file_test() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("./_samples/resolver/Merge1_rest.json")?.load_raw()?;
    Ok(())
  }

  #[test]
  fn read_yaml_file_test() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("./_samples/resolver/Merge1.yaml")?.load_raw()?;
    Ok(())
  }

  #[test]
  #[ignore]
  fn read_beezup_openapi() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("https://api-docs.beezup.com/swagger.json")?.load_raw()?;
    Ok(())
  }
}
