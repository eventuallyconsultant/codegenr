pub mod custom;
pub mod helpers;
pub mod loader;
pub mod processor;
pub mod render;
pub mod resolver;

use std::collections::HashMap;

use loader::*;
use render::*;
use resolver::*;

use handlebars::Handlebars;

#[derive(Debug)]
pub struct Options {
  pub source: String,
  pub output: String,
  pub template: Vec<String>,
  pub intermediate: Option<String>,
  pub custom_helpers: Vec<String>,
  pub global_parameters: HashMap<String, serde_json::Value>,
}

pub fn run_codegenr(options: Options) -> Result<(), anyhow::Error> {
  let document = DocumentPath::parse(&options.source)?;
  let json = resolve_refs(document)?;

  let mut all_templates = vec![];
  for t in options.template {
    let templates = get_templates_from_directory(&t)?;
    all_templates.extend(templates);
  }
  let templates = TemplateCollection::from_list(all_templates)?;

  let mut handlebars = Handlebars::new();
  helpers::handlebars_setup(&mut handlebars, options.global_parameters);
  custom::handlebars_setup(&mut handlebars, options.custom_helpers);
  // todo: custom::handlebars_setup(&mut handlebars);

  let rendered = templates.render(&json, handlebars)?;

  processor::process(&rendered)
}
