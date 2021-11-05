use handlebars::Handlebars;
use serde_json::json;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn some_test() -> Result<(), anyhow::Error> {
    let mut reg = Handlebars::new();
    // render without register
    println!("{}", reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?);

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);

    Ok(())
  }
}
