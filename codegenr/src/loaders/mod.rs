use serde_json::Value;
use thiserror::Error;
use url::Url;

pub mod document_path;
pub use document_path::*;
pub mod csv;
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
  #[error("Could not read `{document}`, every format tried failed:\n{errors}")]
  DeserialisationError {
    document: String,
    /// One `- <format>: <error>` line per format tried.
    errors: String,
  },
  #[error("Yaml error: `{0}`.")]
  YamlError(#[from] serde_yaml::Error),
  #[error("Json error: `{0}`.")]
  JsonError(#[from] serde_json::Error),
  #[error("Xml error: `{0}`.")]
  XmlError(#[from] roxmltree::Error),
  #[error("Graphql error: `{0}`.")]
  GraphqlError(#[from] graphql_parser::schema::ParseError),
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
  /// The content should be csv
  Csv,
  /// The content should be a graphql schema
  Graphql,
  /// We have no f.....g idea
  NoIdea,
}

#[allow(clippy::result_large_err)]
fn json_from_string(content: &str, hint: FormatHint, document: &str) -> Result<Value, LoaderError> {
  use FormatHint::*;
  let formats: &[FormatHint] = match hint {
    Json | NoIdea => &[Json, Yaml, Toml, Graphql],
    Yaml => &[Yaml, Json],
    Toml => &[Toml],
    Xml => &[Xml],
    Csv => &[Csv],
    Graphql => &[Graphql, Json, Yaml, Toml, Xml],
  };
  try_loaders(content, formats).map_err(|errors| LoaderError::DeserialisationError {
    document: document.into(),
    errors,
  })
}

/// The json of the first format that reads the content, or the error of each format tried.
fn try_loaders(content: &str, formats: &[FormatHint]) -> Result<Value, String> {
  let mut errors = Vec::with_capacity(formats.len());
  for format in formats {
    let loaded = match format {
      FormatHint::Json => json::JsonLoader::json_from_str(content).map_err(|e| e.to_string()),
      FormatHint::Yaml => yaml::YamlLoader::json_from_str(content).map_err(|e| e.to_string()),
      FormatHint::Toml => toml::TomlLoader::json_from_str(content).map_err(|e| e.to_string()),
      FormatHint::Xml => xml::XmlLoader::json_from_str(content).map_err(|e| e.to_string()),
      FormatHint::Csv => csv::CsvLoader::json_from_str(content).map_err(|e| e.to_string()),
      FormatHint::Graphql => graphql::GraphqlLoader::json_from_str(content).map_err(|e| e.to_string()),
      FormatHint::NoIdea => todo!(),
    };
    match loaded {
      Ok(json) => return Ok(json),
      Err(error) => errors.push(format!("- {format:?}: {error}")),
    }
  }
  Err(errors.join("\n"))
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

  /// Golden master: the json of each xml sample is committed next to it, order of the keys and formatting of the numbers included.
  #[allow(clippy::result_large_err)]
  #[test_case("./_samples/resolver/plant_catalog.xml", "./_samples/resolver/plant_catalog.json")]
  #[test_case("./_samples/loader/xml_shapes.xml", "./_samples/loader/xml_shapes.json")]
  fn read_xml_file_test(xml_file: &str, expected_json_file: &str) -> Result<(), LoaderError> {
    let loaded = DocumentPath::parse(xml_file)?.load_raw()?;
    let expected = std::fs::read_to_string(expected_json_file)?;
    assert_eq!(format!("{loaded:#}"), expected.trim_end());
    Ok(())
  }

  #[allow(clippy::result_large_err)]
  #[test]
  fn read_csv_file_test() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("./_samples/username.csv")?.load_raw()?;
    dbg!(_result);
    Ok(())
  }

  #[test]
  fn unreadable_content_gives_the_error_of_each_format_tried() {
    let error = json_from_string("a: [1, 2\nb: c\n", FormatHint::Yaml, "bad.yaml").expect_err("The yaml is malformed.");
    assert_eq!(
      error.to_string(),
      "Could not read `bad.yaml`, every format tried failed:\n\
       - Yaml: while parsing a flow sequence, expected ',' or ']' at line 2 column 2\n\
       - Json: expected value at line 1 column 1"
    );
  }

  #[allow(clippy::result_large_err)]
  #[test]
  #[ignore]
  fn read_beezup_openapi() -> Result<(), LoaderError> {
    let _result = DocumentPath::parse("https://api-docs.beezup.com/swagger.json")?.load_raw()?;
    Ok(())
  }
}
