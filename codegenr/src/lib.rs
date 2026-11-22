use crate::errors::CodegenrError;
use serde::Deserialize;
use serde_json::Value;
use std::{
  collections::{BTreeMap, HashMap},
  rc::Rc,
};
use tracing::{error, info};

pub mod custom;
pub mod errors;
pub(crate) mod filesystem;
pub mod helpers;
pub mod loaders;
#[cfg(feature = "bin")]
pub mod opt;
pub mod processor;
pub mod render;
pub mod resolver;
#[cfg(feature = "bin")]
pub mod watch;

use filesystem::save_file_content;
use handlebars::Handlebars;
use thiserror::Error;

pub type OptionsMap = BTreeMap<String, Options>;

type OriginalDocumentsHash = HashMap<loaders::DocumentPath, Rc<Value>>;
type ResolvedDocumentsHash = HashMap<loaders::DocumentPath, Rc<Value>>;

#[derive(Error, Debug)]
pub enum SaverError {
  #[error("Io Error: `{0}`.")]
  Io(#[from] std::io::Error),
}

#[derive(Debug, Deserialize)]
pub struct Options {
  pub source: String,
  pub output: String,
  pub templates: Vec<String>,
  pub intermediate: Option<String>,
  #[serde(default)]
  pub custom_helpers: Vec<String>,
  #[serde(default)]
  pub global_parameters: HashMap<String, serde_json::Value>,
  /// The urls, and the folders out of the current one and the one of `source`, the documents may be read from.
  #[serde(default)]
  pub allowed_sources: Vec<String>,
}

#[::tracing::instrument(level = "trace")]
#[allow(clippy::result_large_err)]
pub fn run_all_codegenr(options_map: OptionsMap) -> Result<(), CodegenrError> {
  let mut original_cache = Default::default();
  let mut errors_count = 0;
  for (name, options) in options_map {
    info!("Running code generation section `{}`", name);
    if let Err(e) = resolve_section(&options, &mut original_cache, &mut Vec::new()).and_then(|json| render_section(&options, &json)) {
      error!("Error while executing the `{}` section: `{}`.", name, e);
      errors_count += 1;
    }
  }

  if errors_count > 0 {
    return Err(CodegenrError::Batch(format!(
      "{errors_count} section(s) failed during the code generation.",
    )));
  }
  Ok(())
}

#[::tracing::instrument(level = "trace")]
#[allow(clippy::result_large_err)]
pub fn run_one_codegenr(options: Options) -> Result<(), errors::CodegenrError> {
  let json = resolve_section(&options, &mut Default::default(), &mut Vec::new())?;
  render_section(&options, &json)
}

/// Loads the source of a section and resolves its `$ref`, the documents read staying in `original_cache` for the next sections.
/// `read` gets these documents, even when the resolution fails: `--watch` watches them.
#[allow(clippy::result_large_err)]
#[::tracing::instrument(level = "trace", skip(original_cache, read))]
fn resolve_section(
  options: &Options,
  original_cache: &mut OriginalDocumentsHash,
  read: &mut Vec<loaders::DocumentPath>,
) -> Result<Rc<Value>, errors::CodegenrError> {
  let document = loaders::DocumentPath::parse(&options.source)?;
  let mut allowed = resolver::AllowedSources::new(&document, &options.allowed_sources)?;
  // A resolved document holds what its `$ref` lead to, read under the `allowed_sources` of its section: no other section reuses it.
  let resolved = resolver::resolve_refs(document, &mut allowed, original_cache, &mut Default::default());
  read.extend(allowed.documents_read().cloned());
  let json = resolved?;

  if options.intermediate.is_some() {
    save_intermediate(&options.intermediate, "resolved.json", &format!("{:#}", json))?;
  }
  Ok(json)
}

/// Renders the templates of a section with its resolved source, and writes the files: `--watch` does it again on each change.
#[allow(clippy::result_large_err)]
#[::tracing::instrument(level = "trace", skip(json))]
fn render_section(options: &Options, json: &Value) -> Result<(), errors::CodegenrError> {
  let mut all_templates = vec![];
  for t in options.templates.iter() {
    let templates = render::get_templates_from_directory(t)?;
    all_templates.extend(templates);
  }

  let mut handlebars = Handlebars::new();
  helpers::handlebars_misc_setup(&mut handlebars);
  helpers::handlebars_stateless_setup(&mut handlebars);
  helpers::handlebars_statefull_setup(&mut handlebars, options.global_parameters.clone());
  custom::handlebars_setup(&mut handlebars, &options.custom_helpers)?;

  let templates = render::TemplateCollection::from_list(all_templates)?;
  templates.setup_handlebars(&mut handlebars)?;

  let main_template_name = templates.main_template_name().to_owned();

  let rendered = handlebars.render(&main_template_name, json)?;

  save_intermediate(&options.intermediate, "rendered.txt", &rendered)?;

  let report = processor::process(&rendered, options.output.clone())?;
  info!(
    "{} file(s) written, {} unchanged, in `{}`.",
    report.written_files, report.unchanged_files, options.output
  );
  Ok(())
}

fn save_intermediate(file: &Option<String>, extension: &str, content: &str) -> Result<(), SaverError> {
  if let Some(s) = file {
    let full_file_name = format!("{}.{}", s, extension);
    save_file_content(".", &full_file_name, content)?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Write;
  use tempfile::TempDir;

  #[test]
  fn custom_helpers_and_global_parameters_are_optional() -> anyhow::Result<()> {
    let sections: OptionsMap = toml::from_str("[api]\nsource = \"api.yaml\"\noutput = \"out\"\ntemplates = [\"_templates\"]\n")?;
    let api = sections
      .get("api")
      .ok_or_else(|| anyhow::anyhow!("The `api` section is missing."))?;
    assert!(api.custom_helpers.is_empty());
    assert!(api.global_parameters.is_empty());
    Ok(())
  }

  /// What `--watch` does on a change: a section renders again from its source resolved once, which is not read again.
  #[test]
  fn a_section_renders_again_from_its_resolved_source() -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("LIB_tests")?;
    let root = tmp.path();
    let write = |path: &str, content: &str| -> anyhow::Result<()> {
      let (mut file, _) = filesystem::create_file(root, path)?;
      file.write_all(content.as_bytes())?;
      Ok(())
    };
    write("api.yaml", "name: World\n")?;
    write("templates/main.hbs", "### FILE hello.txt\nHello {{name}}\n### /FILE\n")?;
    let options = Options {
      source: root.join("api.yaml").display().to_string(),
      output: root.join("out").display().to_string(),
      templates: vec![root.join("templates").display().to_string()],
      intermediate: None,
      custom_helpers: vec![],
      global_parameters: Default::default(),
      allowed_sources: vec![],
    };
    let json = resolve_section(&options, &mut Default::default(), &mut Vec::new())?;
    render_section(&options, &json)?;
    assert_eq!(std::fs::read_to_string(root.join("out").join("hello.txt"))?, "Hello World\n");

    std::fs::remove_file(root.join("api.yaml"))?;
    write("templates/main.hbs", "### FILE hello.txt\nBonjour {{name}}\n### /FILE\n")?;
    render_section(&options, &json)?;

    assert_eq!(std::fs::read_to_string(root.join("out").join("hello.txt"))?, "Bonjour World\n");
    Ok(())
  }

  /// The sections share the documents they read, but each one gets only what its `allowed_sources` let it read.
  #[test]
  fn a_section_does_not_get_what_another_section_allowed() -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("LIB_tests")?;
    let root = tmp.path();
    for (path, content) in [
      ("specs/api.yaml", "a:\n  $ref: '../secret/token.yaml'\n"),
      ("secret/token.yaml", "token: abc\n"),
      ("templates/main.hbs", "### FILE token.txt\n{{a.token}}\n### /FILE\n"),
    ] {
      let (mut file, _) = filesystem::create_file(root, path)?;
      file.write_all(content.as_bytes())?;
    }
    let section = |output: &str, allowed_sources: Vec<String>| Options {
      source: root.join("specs/api.yaml").display().to_string(),
      output: root.join(output).display().to_string(),
      templates: vec![root.join("templates").display().to_string()],
      intermediate: None,
      custom_helpers: vec![],
      global_parameters: Default::default(),
      allowed_sources,
    };
    let secret = root.join("secret").display().to_string();
    // Run in this order: the allowing section first fills the caches.
    let sections = OptionsMap::from([
      ("a_allowing".into(), section("out_a", vec![secret])),
      ("b_not_allowing".into(), section("out_b", vec![])),
    ]);

    let error = run_all_codegenr(sections).expect_err("The second section does not allow the secret folder.");

    assert!(matches!(error, CodegenrError::Batch(_)), "{error}");
    assert_eq!(std::fs::read_to_string(root.join("out_a").join("token.txt"))?, "abc\n");
    assert!(!root.join("out_b").join("token.txt").exists());
    Ok(())
  }
}
