pub mod custom;
pub(crate) mod filesystem;
pub mod helpers;
pub mod loader;
pub mod processor;
pub mod render;
pub mod resolver;

use std::{collections::HashMap, fs::File, io::Write};

use handlebars::Handlebars;
use loader::*;
use render::*;
use resolver::*;

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
  let document = DocumentPath::parse(&options.source)?;
  let json = resolve_refs(document)?;

  if options.intermediate.is_some() {
    save_intermediate(&options.intermediate, "resolved.json", &json.to_string())?;
  }

  let mut all_templates = vec![];
  for t in options.templates {
    let templates = get_templates_from_directory(&t)?;
    all_templates.extend(templates);
  }
  let templates = TemplateCollection::from_list(all_templates)?;

  let mut handlebars = Handlebars::new();
  helpers::handlebars_setup(&mut handlebars, options.global_parameters);
  custom::handlebars_setup(&mut handlebars, options.custom_helpers)?;

  let rendered = templates.render(&json, handlebars)?;

  save_intermediate(&options.intermediate, "rendered.txt", &rendered)?;

  processor::process(&rendered, options.output)
}

fn save_intermediate(file: &Option<String>, extension: &str, content: &str) -> Result<(), anyhow::Error> {
  if let Some(s) = file {
    let full_file_name = format!("{}.{}", s, extension);
    let mut f = File::create(full_file_name)?;
    f.write_all(content.as_bytes())?;
  }
  Ok(())
}
