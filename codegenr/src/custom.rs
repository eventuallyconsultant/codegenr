use std::path::Path;

use handlebars::Handlebars;

pub fn handlebars_setup(handlebars: &mut Handlebars) -> Result<(), anyhow::Error> {
  handlebars_add_script(handlebars, "./_samples/rhai/param_0_len.rhai")?;
  Ok(())
}
pub fn handlebars_add_script(handlebars: &mut Handlebars, script_file: &str) -> Result<(), anyhow::Error> {
  let name = Path::new(script_file)
    .file_stem()
    .ok_or_else(|| anyhow::anyhow!("File path passed has no file stem."))?
    .to_str()
    .ok_or_else(|| anyhow::anyhow!("Error converting OsStr to str."))?;
  handlebars.register_script_helper_file(name, script_file)?;
  Ok(())
}

pub fn exec_template(json: serde_json::Value, template: &str) -> String {
  let mut h = Handlebars::new();
  handlebars_setup(&mut h).expect("Could not setup handlebars.");
  h.register_template_string("test", template).expect("Could not register template.");
  h.render("test", &json).expect("Template render returned an error.")
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json::json;
  // use test_case::test_case;

  #[test]
  fn t() {
    assert_eq!(exec_template(json!({"key": "value"}), "{{param_0_len \"plop\"}}"), "4")
  }
}
