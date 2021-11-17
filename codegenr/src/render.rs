use handlebars::Handlebars;
use serde_json::json;
use walkdir::WalkDir;

const PARTIAL_TEMPLATE_PREFIX: &str = "_";
const HANDLEBARS_TEMPLATE_EXTENSION: &str = ".hbs";

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TemplateType {
  Main,
  Partial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
  pub template_type: TemplateType,
  pub file_name: String,
  pub file_path: String,
}

pub fn get_templates_from_directory(dir_path: &str) -> Result<Vec<Template>, anyhow::Error> {
  let mut result = vec![];
  for entry in WalkDir::new(dir_path) {
    let entry = entry?;
    dbg!(&entry);

    let file_path = entry.path().to_str();
    let file_path = match file_path {
      Some(f) => f,
      None => continue,
    };

    let file_name = entry.file_name().to_str();
    if let Some(f_name) = file_name {
      if !f_name.ends_with(HANDLEBARS_TEMPLATE_EXTENSION) {
        continue;
      }
      result.push(Template {
        file_name: f_name.to_string(),
        file_path: file_path.to_string(),
        template_type: if f_name.starts_with(PARTIAL_TEMPLATE_PREFIX) {
          TemplateType::Partial
        } else {
          TemplateType::Main
        },
      })
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
    assert_eq!(
      templates,
      vec!(Template {
        template_type: TemplateType::Main,
        file_name: "".into(),
        file_path: Default::default(),
      })
    );
    Ok(())
  }
}
