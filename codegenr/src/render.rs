use handlebars::Handlebars;
use serde_json::json;
use std::collections::HashMap;
use walkdir::WalkDir;

const PARTIAL_TEMPLATE_PREFIX: &str = "_";
const HANDLEBARS_TEMPLATE_EXTENSION: &str = ".hbs";

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateCollection {
  main: Template,
  partials: HashMap<String, Template>,
}

impl TemplateCollection {
  pub fn from_list(templates: impl IntoIterator<Item = Template>) -> Result<TemplateCollection, anyhow::Error> {
    let mut main: Option<Template> = None;
    let mut partials = HashMap::<String, Template>::new();

    for t in templates {
      match t.template_type() {
        TemplateType::Main => {
          if let Some(existing) = main.as_ref() {
            return Err(anyhow::anyhow!(
              "2 main templates were found : \n-{}\n-{}\nTheir should be only one in all the template directories",
              existing.file_path(),
              t.file_path()
            ));
          };
          main = Some(t);
        }
        TemplateType::Partial => {
          if let Some(existing) = partials.get(t.template_name()) {
            return Err(anyhow::anyhow!(
              "2 partial templates are named '{}' : \n-{}\n-{}\nThey should have unique names",
              existing.template_name(),
              existing.file_path(),
              t.file_path()
            ));
          };
          partials.insert(t.template_name().into(), t);
        }
      }
    }

    let main = main.ok_or_else(|| anyhow::anyhow!("No main template has been detected, we don't know what to execute..."))?;

    Ok(Self { main, partials })
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

pub fn get_templates_from_directory(dir_path: &str) -> Result<Vec<Template>, anyhow::Error> {
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

  #[test]
  fn some_handlebars_first_test() -> Result<(), anyhow::Error> {
    let mut reg = Handlebars::new();
    // render without register
    println!("{}", reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?);

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);

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
    assert!(err.to_string().starts_with("2 partial templates are named 'plop'"));
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
  fn from_list_success_main_only() {
    let list = vec![
      Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
      Template::new(TemplateType::Partial, "_partial.hbs", "./_samples/render/templates/_partial.hbs"),
    ];
    let test = TemplateCollection::from_list(list).expect("?");
    let mut map = HashMap::new();
    map.insert(
      "partial".into(),
      Template::new(TemplateType::Main, "plop.hbs", "./_samples/render/templates/sub/plop.hbs"),
    );

    let expected = TemplateCollection {
      main: Template {
        template_type: crate::render::TemplateType::Main,
        file_name: "plop.hbs".to_string(),
        file_path: "./_samples/render/templates/sub/plop.hbs".to_string(),
      },
      partials: map,
    };
    dbg!(test);

    //assert_eq!(test, expected);
  }
}
