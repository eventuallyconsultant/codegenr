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
  #[error("No document to load: its path is empty.")]
  EmptyPath,
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
}

#[allow(clippy::result_large_err)]
/// `hint` is `None` when the path says nothing of the format.
fn json_from_string(content: &str, hint: Option<FormatHint>, document: &str) -> Result<Value, LoaderError> {
  use FormatHint::*;
  // A UTF-8 BOM, which editors and Windows tools often write: json refuses it, yaml keeps it in the first key.
  let content = content.strip_prefix('\u{feff}').unwrap_or(content);
  let formats: &[FormatHint] = match hint {
    Some(Json) | None => &[Json, Yaml, Toml, Graphql],
    Some(Yaml) => &[Yaml, Json],
    Some(Toml) => &[Toml],
    Some(Xml) => &[Xml],
    Some(Csv) => &[Csv],
    Some(Graphql) => &[Graphql, Json, Yaml, Toml, Xml],
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
    let error = json_from_string("a: [1, 2\nb: c\n", Some(FormatHint::Yaml), "bad.yaml").expect_err("The yaml is malformed.");
    assert_eq!(
      error.to_string(),
      "Could not read `bad.yaml`, every format tried failed:\n\
       - Yaml: did not find expected ',' or ']' at line 2 column 2, while parsing a flow sequence at line 1 column 4\n\
       - Json: expected value at line 1 column 1"
    );
  }

  /// Editors and Windows tools often start a file with a UTF-8 BOM, which used to fail the json loader.
  #[test_case("\u{feff}{\"a\": 1}", Some(FormatHint::Json) ; "json")]
  #[test_case("\u{feff}{\"a\": 1}", None ; "json without extension")]
  #[test_case("\u{feff}a: 1", Some(FormatHint::Yaml) ; "yaml")]
  #[test_case("\u{feff}a = 1", Some(FormatHint::Toml) ; "toml")]
  fn a_source_starting_with_a_bom_is_read(content: &str, hint: Option<FormatHint>) -> anyhow::Result<()> {
    assert_eq!(json_from_string(content, hint, "bom")?, serde_json::json!({ "a": 1 }));
    Ok(())
  }

  /// An empty `source` in `codegenr.toml` used to panic.
  #[test]
  fn loading_an_empty_path_is_an_error() -> anyhow::Result<()> {
    let error = DocumentPath::parse("")?.load_raw().expect_err("There is no document to load.");
    assert_eq!(error.to_string(), "No document to load: its path is empty.");
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
