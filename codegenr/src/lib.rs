pub mod custom;
pub mod helpers;
pub mod loader;
pub mod processor;
pub mod render;
pub mod resolver;

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
  pub customhelpers: Vec<String>,
  pub globalparameter: Vec<String>,
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
  helpers::handlebars_setup(&mut handlebars, todo!());
  // custom::handlebars_setup(&mut handlebars);

  let rendered = templates.render(&json, handlebars)?;

  println!("{}", rendered);
  todo!("output the rendered string!")
}
