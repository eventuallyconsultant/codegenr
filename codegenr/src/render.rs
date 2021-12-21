use handlebars::{Handlebars, TemplateError};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use walkdir::WalkDir;

const PARTIAL_TEMPLATE_PREFIX: &str = "_";
const HANDLEBARS_TEMPLATE_EXTENSION: &str = ".hbs";

#[derive(Error, Debug)]
pub enum RenderError {
  #[error("TemplateRender error: {0}")]
  RenderTemp(#[from] handlebars::RenderError),
  #[error("Template error: {0}")]
  Template(#[from] TemplateError),
  #[error("Walkdir error: {0}")]
  Walkdir(#[from] walkdir::Error),
  #[error("Template error: {0}")]
  TwoMainTemp(&'static str),
  #[error("Template error: {0}")]
  UniqueNameTemp(&'static str),
  #[error("Template error: {0}")]
  NoMainTemp(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateCollection {
  main: Template,
  partials: HashMap<String, Template>,
}

impl TemplateCollection {
  pub fn from_list(templates: impl IntoIterator<Item = Template>) -> Result<TemplateCollection, RenderError> {
    let mut main: Option<Template> = None;
    let mut partials = HashMap::<String, Template>::new();

    for t in templates {
      match t.template_type() {
        TemplateType::Main => {
          if let Some(existing) = main.as_ref() {
            return Err(RenderError::TwoMainTemp(
              "2 main templates were found, their should be only one in all the template directories",
            ));
          };
          main = Some(t);
        }
        TemplateType::Partial => {
          if let Some(existing) = partials.get(t.template_name()) {
            return Err(RenderError::UniqueNameTemp(
              "2 partial templates are named, they should have unique names",
            ));
          };
          partials.insert(t.template_name().into(), t);
        }
      }
    }

    let main = main.ok_or_else(|| RenderError::NoMainTemp("No main template has been detected, we don't know what to execute..."))?;

    Ok(Self { main, partials })
  }

  pub fn render(&self, json: &Value, mut handlebars: Handlebars) -> Result<String, RenderError> {
    let template_name = self.main.template_name();
    handlebars.register_template_file(template_name, self.main.file_path())?;
    for (_, value) in self.partials.iter() {
      handlebars.register_template_file(value.template_name(), value.file_path())?
    }
    Ok(handlebars.render(template_name, json)?)
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TemplateType {
  Main,
  Partial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
  template_type: TemplateType,
  file_name: String,
  file_path: String,
}

impl Template {
  pub fn new(template_type: TemplateType, file_name: impl Into<String>, file_path: impl Into<String>) -> Self {
    Self {
      template_type,
      file_name: file_name.into(),
      file_path: file_path.into(),
    }
  }

  pub fn file_path(&self) -> &str {
    &self.file_path
  }

  pub fn template_name(&self) -> &str {
    self.file_name.trim_start_matches('_').trim_end_matches(".hbs")
  }

  pub fn template_type(&self) -> TemplateType {
    self.template_type
  }
}

pub fn get_templates_from_directory(dir_path: &str) -> Result<Vec<Template>, RenderError> {
  let mut result = vec![];
  for entry in WalkDir::new(dir_path) {
    let entry = entry?;

    let file_path = entry.path().to_str();
    let file_path = match file_path {
      Some(f) => f,
      None => continue,
    };

    let file_name = entry.file_name().to_str();
    if let Some(file_name) = file_name {
      if !file_name.ends_with(HANDLEBARS_TEMPLATE_EXTENSION) {
        continue;
      }
      let t = if file_name.starts_with(PARTIAL_TEMPLATE_PREFIX) {
        TemplateType::Partial
      } else {
        TemplateType::Main
      };
      result.push(Template::new(t, file_name, file_path));
    }
  }
  Ok(result)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::helpers::handlebars_setup;
  use crate::loader::DocumentPath;
  use crate::resolver::resolve_refs;

  #[test]
  fn handlebars_loading_test() -> Result<(), anyhow::Error> {
    let document = DocumentPath::parse("_samples/resolver/petshop_with_external.yaml")?;
    let json = resolve_refs(document)?;
    let list = get_templates_from_directory("_samples/render/test_denis")?;
    let collection = TemplateCollection::from_list(list)?;

    let mut h = Handlebars::new();
    handlebars_setup(&mut h, Default::default());
    let result = collection.render(&json, h)?;
    dbg!(result);

    Ok(())
  }

  #[test]
  fn get_templates_from_directory_test() -> Result<(), anyhow::Error> {
    let mut templates = get_templates_from_directory("./_samples/render/templates")?;
    templates.sort_by_key(|t| t.file_path.clone());
    let expected = vec![
      Template::new(
        TemplateType::Partial,
        "_other_partial.hbs",
        "./_samples/render/templates/_other_partial.hbs",
      ),
      Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
      Template::new(TemplateType::Partial, "_plop.hbs", "./_samples/render/templates/sub/_plop.hbs"),
      Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
      Template::new(TemplateType::Main, "test.hbs", "./_samples/render/templates/test.hbs"),
    ];
    // dbg!(&templates, &expected);
    assert_eq!(templates, expected);

    let first = templates.get(0).expect("?");
    assert_eq!(first.template_name(), "other_partial");
    Ok(())
  }

  #[test]
  fn from_list_fails_with_double_main() {
    let list = vec![
      Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
      Template::new(TemplateType::Main, "test.hbs", "./_samples/render/templates/test.hbs"),
    ];

    let test = TemplateCollection::from_list(list);
    let err = test.expect_err("Should be an error");
    assert!(err.to_string().starts_with("2 main templates were found"));
  }

  #[test]
  fn from_list_fails_with_same_names_partials() {
    let list = vec![
      Template::new(
        TemplateType::Partial,
        "_other_partial.hbs",
        "./_samples/render/templates/_other_partial.hbs",
      ),
      Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
      Template::new(TemplateType::Partial, "_plop.hbs", "./_samples/render/templates/sub/_plop.hbs"),
      Template::new(TemplateType::Partial, "_plop.hbs", "./_samples/render/templates/sub2/_plop.hbs"),
    ];

    let test = TemplateCollection::from_list(list);
    let err = test.expect_err("Should be an error");
    assert!(err.to_string().starts_with("2 partial templates are named `plop`"));
  }

  #[test]
  fn from_list_fails_with_no_main_found() {
    let list = vec![
      Template::new(
        TemplateType::Partial,
        "_other_partial.hbs",
        "./_samples/render/templates/_other_partial.hbs",
      ),
      Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
      Template::new(TemplateType::Partial, "_plop.hbs", "./_samples/render/templates/sub/_plop.hbs"),
    ];

    let test = TemplateCollection::from_list(list);
    let err = test.expect_err("Should be an error");
    assert!(err
      .to_string()
      .starts_with("No main template has been detected, we don't know what to execute..."));
  }

  #[test]
  fn from_list_success() {
    let list = vec![
      Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
      Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
    ];
    let test = TemplateCollection::from_list(list).expect("?");
    let mut map = HashMap::new();
    map.insert(
      "partial".into(),
      Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
    );
    let expected = TemplateCollection {
      main: Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
      partials: map,
    };

    assert_eq!(test, expected);
  }
}
