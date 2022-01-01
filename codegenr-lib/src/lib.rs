use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, rc::Rc};
use tracing::{error, info};

pub mod custom;
pub mod errors;
pub(crate) mod filesystem;
pub mod helpers;
pub mod loader;
pub mod processor;
pub mod render;
pub mod resolver;

use filesystem::save_file_content;
use handlebars::Handlebars;
use thiserror::Error;

type OptionsMap = HashMap<String, Options>;

type OriginalDocumentsHash = HashMap<loader::DocumentPath, Rc<Value>>;
type ResolvedDocumentsHash = HashMap<loader::DocumentPath, Rc<Value>>;
type HandlebarsHash<'a> = HashMap<HandlebarsReusableConf, (String, Handlebars<'a>)>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct HandlebarsReusableConf {
  pub templates: Vec<String>,
  pub custom_helpers: Vec<String>,
}

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
pub fn run_all_codegenr(options_map: OptionsMap) -> Result<(), errors::CodegenrError> {
  let mut original_cache = Default::default();
  let mut resolved_cache = Default::default();
  let mut reusables = Default::default();
  for (name, options) in options_map {
    info!("Running code generation section `{}`", name);
    if let Err(e) = run_codegenr(options, &mut original_cache, &mut resolved_cache, &mut reusables) {
      error!("Error while executing the `{}` section: `{}`.", name, e);
    }
  }
  Ok(())
}

#[::tracing::instrument(level = "trace")]
pub fn run_one_codegenr(options: Options) -> Result<(), errors::CodegenrError> {
  let mut original_cache = Default::default();
  let mut resolved_cache = Default::default();
  let mut reusables = Default::default();
  run_codegenr(options, &mut original_cache, &mut resolved_cache, &mut reusables)
}

#[::tracing::instrument(level = "trace")]
fn run_codegenr(
  options: Options,
  original_cache: &mut OriginalDocumentsHash,
  resolved_cache: &mut ResolvedDocumentsHash,
  reusables: &mut HandlebarsHash,
) -> Result<(), errors::CodegenrError> {
  let document = loader::DocumentPath::parse(&options.source)?;
  let json = resolver::resolve_refs(document, original_cache, resolved_cache)?;

  if options.intermediate.is_some() {
    save_intermediate(&options.intermediate, "resolved.json", &format!("{:#}", json))?;
  }

  let (main_template_name, mut handlebars) = reusables
    .entry(HandlebarsReusableConf {
      custom_helpers: options.custom_helpers,
      templates: options.templates,
    })
    .or_insert_with_key(|conf| {
      let mut all_templates = vec![];
      for t in conf.templates.iter() {
        let templates = render::get_templates_from_directory(t).unwrap(); //?;
        all_templates.extend(templates);
      }
      let templates = render::TemplateCollection::from_list(all_templates).unwrap(); //?;

      let mut handlebars = Handlebars::new();
      templates.setup_handlebars(&mut handlebars).unwrap(); // todo?;
      custom::handlebars_setup(&mut handlebars, &conf.custom_helpers).unwrap(); //?;
      (templates.main_template_name().to_owned(), handlebars)
    })
    .clone();

  helpers::handlebars_setup(&mut handlebars, options.global_parameters);

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
