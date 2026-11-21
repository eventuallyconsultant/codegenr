use handlebars::{Handlebars, TemplateError};
use std::collections::HashMap;
use thiserror::Error;
use walkdir::WalkDir;

const PARTIAL_TEMPLATE_PREFIX: &str = "_";
const HANDLEBARS_TEMPLATE_EXTENSION: &str = ".hbs";

#[derive(Error, Debug)]
pub enum RenderError {
  #[error("Template error: `{0}`.")]
  Template(#[from] TemplateError),
  #[error("Walkdir error: `{0}`.")]
  Walkdir(#[from] walkdir::Error),
  #[error("2 main templates were found : `{0}` and `{1}`. There should be only one in all the template directories.")]
  TwoMainTemp(String, String),
  #[error("2 partial templates are named `{0}` they should have unique names.")]
  UniqueNameTemp(String),
  #[error("No main template has been detected, we don't know what to execute.")]
  NoMainTemp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateCollection {
  main: Template,
  partials: HashMap<String, Template>,
}

impl TemplateCollection {
  #[allow(clippy::result_large_err)]
  pub fn from_list(templates: impl IntoIterator<Item = Template>) -> Result<TemplateCollection, RenderError> {
    let mut main: Option<Template> = None;
    let mut partials = HashMap::<String, Template>::new();

    for t in templates {
      match t.template_type() {
        TemplateType::Main => {
          if let Some(existing) = main.as_ref() {
            return Err(RenderError::TwoMainTemp(
              existing.template_name().to_string(),
              t.template_name().to_string(),
            ));
          };
          main = Some(t);
        }
        TemplateType::Partial => {
          if let Some(existing) = partials.get(t.template_name()) {
            return Err(RenderError::UniqueNameTemp(existing.template_name().into()));
          };
          partials.insert(t.template_name().into(), t);
        }
      }
    }

    let main = main.ok_or(RenderError::NoMainTemp)?;

    Ok(Self { main, partials })
  }

  #[allow(clippy::result_large_err)]
  pub fn setup_handlebars(&self, handlebars: &mut Handlebars) -> Result<(), RenderError> {
    handlebars.register_template_file(self.main.template_name(), self.main.file_path())?;
    for (_, value) in self.partials.iter() {
      handlebars.register_template_file(value.template_name(), value.file_path())?
    }
    Ok(())
  }

  pub fn main_template_name(&self) -> &str {
    self.main.template_name()
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

#[allow(clippy::result_large_err)]
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
mod tests {
  use super::*;
  use crate::helpers::*;
  use serde_json::json;
  use test_case::test_case;

  /// Guards any handlebars bump, every case passes on 5.1.2.
  /// handlebars 6.3.2 loses the `@key`, `@index`, `@first` and `@last` of an `#each` in the partials it calls,
  /// fixed upstream by 6.4.2 (https://github.com/sunng87/handlebars-rust/pull/758).
  /// 6.4.4 still loses the parent context, `../`, in those partials: the two `dot dot` cases fail, no upstream issue yet (2026-11-21).
  #[test_case("{{#each object}}{{> part}}{{/each}}", "{{@key}}={{this}};", "a=1;b=2;" ; "key of an object")]
  #[test_case("{{#each array}}{{> part}}{{/each}}", "{{@index}}:{{this}};", "0:x;1:y;" ; "index of an array")]
  #[test_case("{{#each array}}{{> part}}{{/each}}", "{{#if @first}}first {{/if}}{{this}}{{#if @last}} last{{/if}};", "first x;y last;" ; "first and last")]
  #[test_case("{{#each object}}{{> part name=@key}}{{/each}}", "{{name}};", "a;b;" ; "key passed as a hash parameter")]
  #[test_case("{{#each object}}{{> part}}{{/each}}", "{{> sub}}", "a;b;" ; "key read in a partial of a partial")]
  #[test_case("{{#each object}}{{#with_set \"name\" @key}}{{> part}}{{/with_set}}{{/each}}", "{{get \"name\"}};", "a;b;" ; "key stored by with_set")]
  #[test_case("{{#each object}}{{> part}}{{/each}}", "{{@key}}:{{../flag}};", "a:yes;b:yes;" ; "parent read with dot dot")]
  #[test_case(
    "{{#each object}}{{> part}}{{/each}}",
    "{{#if (is_oapi3_property_required @key ../required)}}{{@key}} is required;{{/if}}",
    "a is required;" ;
    "parent read with dot dot by a helper"
  )]
  fn partials_called_from_an_each_see_its_variables(main: &str, partial: &str, expected: &str) -> anyhow::Result<()> {
    let mut h = Handlebars::new();
    handlebars_misc_setup(&mut h);
    handlebars_stateless_setup(&mut h);
    handlebars_statefull_setup(&mut h, Default::default());
    h.register_template_string("main", main)?;
    h.register_template_string("part", partial)?;
    h.register_template_string("sub", "{{@key}};")?;
    let rendered = h.render(
      "main",
      &json!({ "object": { "a": 1, "b": 2 }, "array": ["x", "y"], "flag": "yes", "required": ["a"] }),
    )?;
    assert_eq!(rendered, expected);
    Ok(())
  }

  #[test]
  fn handlebars_loading_test() -> Result<(), anyhow::Error> {
    let list = get_templates_from_directory("_samples/render/test_denis")?;
    let collection = TemplateCollection::from_list(list)?;
    let mut h = Handlebars::new();
    handlebars_stateless_setup(&mut h);
    handlebars_statefull_setup(&mut h, Default::default());
    collection.setup_handlebars(&mut h)?;
    Ok(())
  }

  #[test]
  fn get_templates_from_directory_test() -> Result<(), anyhow::Error> {
    let mut templates = get_templates_from_directory("./_samples/render/templates")?;
    templates.sort_by_key(|t| t.file_path.clone());
    let expected = if cfg!(windows) {
      vec![
        Template::new(
          TemplateType::Partial,
          "_other_partial.hbs",
          "./_samples/render/templates\\_other_partial.hbs",
        ),
        Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates\\_partial.hbs"),
        Template::new(TemplateType::Partial, "_plop.hbs", "./_samples/render/templates\\sub\\_plop.hbs"),
        Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates\\sub\\plop.hbs"),
        Template::new(TemplateType::Main, "test.hbs", "./_samples/render/templates\\test.hbs"),
      ]
    } else {
      vec![
        Template::new(
          TemplateType::Partial,
          "_other_partial.hbs",
          "./_samples/render/templates/_other_partial.hbs",
        ),
        Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
        Template::new(TemplateType::Partial, "_plop.hbs", "./_samples/render/templates/sub/_plop.hbs"),
        Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
        Template::new(TemplateType::Main, "test.hbs", "./_samples/render/templates/test.hbs"),
      ]
    };

    // dbg!(&templates, &expected);
    assert_eq!(templates, expected);

    let first = templates.first().expect("?");
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
    assert_eq!(
      err.to_string(),
      "2 main templates were found : `plop` and `test`. There should be only one in all the template directories."
    );
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
    assert_eq!(
      err.to_string(),
      "2 partial templates are named `plop` they should have unique names."
    );
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
    assert!(matches!(err, RenderError::NoMainTemp));
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
