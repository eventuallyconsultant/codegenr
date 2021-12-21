pub mod custom;
pub mod errors;
pub(crate) mod filesystem;
pub mod helpers;
pub mod loader;
pub mod processor;
pub mod render;
pub mod resolver;

use std::collections::HashMap;

use errors::CodegenrError;
use filesystem::save_file_content;
use handlebars::Handlebars;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SaverError {
  #[error("Io Error: `{0}`.")]
  Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Options {
  pub source: String,
  pub output: String,
  pub templates: Vec<String>,
  pub intermediate: Option<String>,
  pub custom_helpers: Vec<String>,
  pub global_parameters: HashMap<String, serde_json::Value>,
}

pub fn run_codegenr(options: Options) -> Result<(), anyhow::Error> {
  let document = loader::DocumentPath::parse(&options.source)?;
  let json = resolver::resolve_refs(document)?;

  if options.intermediate.is_some() {
    save_intermediate(&options.intermediate, "resolved.json", &format!("{:#}", json))?;
  }

  let mut all_templates = vec![];
  for t in options.templates {
    let templates = render::get_templates_from_directory(&t)?;
    all_templates.extend(templates);
  }
  let templates = render::TemplateCollection::from_list(all_templates)?;

  let mut handlebars = Handlebars::new();
  helpers::handlebars_setup(&mut handlebars, options.global_parameters);
  custom::handlebars_setup(&mut handlebars, options.custom_helpers)?;

  let rendered = templates.render(&json, handlebars)?;

  save_intermediate(&options.intermediate, "rendered.txt", &rendered)?;

  processor::process(&rendered, options.output)
}

fn save_intermediate(file: &Option<String>, extension: &str, content: &str) -> Result<(), SaverError> {
  if let Some(s) = file {
    let full_file_name = format!("{}.{}", s, extension);
    save_file_content(".", &full_file_name, content)?;
  }
  Ok(())
}
