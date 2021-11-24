use handlebars::handlebars_helper;
use handlebars::Handlebars;

pub fn register_custom_helpers(handlebars: &mut Handlebars) {
  handlebars.register_helper("hex", Box::new(Hex));
  handlebars.register_helper("trim", Box::new(trim));
  handlebars.register_helper("UppercaseFirstLetter", Box::new(UppercaseFirstLetter));
  handlebars.register_helper("ToLowerCase", Box::new(ToLowerCase));
  handlebars.register_helper("StartWith", Box::new(StartWith));
}

/// Return the hexadecimal representation of the integer value
/// ```
/// # use codegenr::custom_helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(serde_json::Value::Null, "{{hex 42}}"),
///   "0x2a"
/// );
/// assert_eq!(
///   test_helper(json!({ "value": 42 }), "{{hex value}}"),
///   "0x2a"
/// );
/// ```
pub fn hex(v: i64) -> String {
  format!("0x{:x}", v)
}

handlebars_helper!(Hex: |v: i64| hex(v));

handlebars_helper!(trim: |v: String| v.trim());

handlebars_helper!(UppercaseFirstLetter: |v: String| captitalize_first_letter(v));

handlebars_helper!(ToLowerCase: |v: String| v.to_lowercase());

//handlebars_helper!(IfEmpty: |v: String| CheckIfEmpty(v));

handlebars_helper!(IfNotEmpty: |v: String| check_if_not_empty(v));

handlebars_helper!(StartWith: |v: String| check_if_start_with(v));

// handlebars_helper!();

pub fn check_if_not_empty(v: String) -> String {
  todo!()
}

pub fn check_if_start_with(v: String) -> String {
  todo!()
}

pub fn captitalize_first_letter(v: String) -> String {
  let mut ve: Vec<char> = v.chars().collect();
  ve[0] = ve[0].to_uppercase().next().unwrap();
  let result: String = ve.into_iter().collect();
  result
}

// #[cfg(doctest)]
pub fn test_helper(json: serde_json::Value, template: &str) -> String {
  let mut h = Handlebars::new();
  register_custom_helpers(&mut h);
  h.register_template_string("test", template).expect("OK!");
  h.render("test", &json).unwrap()
}
