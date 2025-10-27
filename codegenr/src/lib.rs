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
  pub custom_helpers: Vec<String>,
  pub global_parameters: HashMap<String, serde_json::Value>,
}

#[::tracing::instrument(level = "trace")]
#[allow(clippy::result_large_err)]
pub fn run_all_codegenr(options_map: OptionsMap) -> Result<(), CodegenrError> {
  let mut original_cache = Default::default();
  let mut resolved_cache = Default::default();
  let mut errors_count = 0;
  for (name, options) in options_map {
    info!("Running code generation section `{}`", name);
    if let Err(e) = run_codegenr(options, &mut original_cache, &mut resolved_cache) {
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
  let mut original_cache = Default::default();
  let mut resolved_cache = Default::default();
  run_codegenr(options, &mut original_cache, &mut resolved_cache)
}

#[allow(clippy::result_large_err)]
#[::tracing::instrument(level = "trace")]
fn run_codegenr(
  options: Options,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
) -> Result<(), errors::CodegenrError> {
  let document = loaders::DocumentPath::parse(&options.source)?;
  let json = resolver::resolve_refs(document, original_cache, resolved_cache)?;

  if options.intermediate.is_some() {
    save_intermediate(&options.intermediate, "resolved.json", &format!("{:#}", json))?;
  }

  let mut all_templates = vec![];
  for t in options.templates.iter() {
    let templates = render::get_templates_from_directory(t)?;
    all_templates.extend(templates);
  }

  let mut handlebars = Handlebars::new();
  helpers::handlebars_misc_setup(&mut handlebars);
  helpers::handlebars_stateless_setup(&mut handlebars);
  helpers::handlebars_statefull_setup(&mut handlebars, options.global_parameters);
  custom::handlebars_setup(&mut handlebars, &options.custom_helpers)?;

  let templates = render::TemplateCollection::from_list(all_templates)?;
  templates.setup_handlebars(&mut handlebars)?;

  let main_template_name = templates.main_template_name().to_owned();

  let rendered = handlebars.render(&main_template_name, &(*json))?;

  save_intermediate(&options.intermediate, "rendered.txt", &rendered)?;

  processor::process(&rendered, options.output)?;
  Ok(())
}

fn save_intermediate(file: &Option<String>, extension: &str, content: &str) -> Result<(), SaverError> {
  if let Some(s) = file {
    let full_file_name = format!("{}.{}", s, extension);
    save_file_content(".", &full_file_name, content)?;
  }
  Ok(())
}
