use handlebars::handlebars_helper;
use handlebars::Handlebars;

pub fn register_custom_helpers(handlebars: &mut Handlebars) {
  handlebars.register_helper("hex", Box::new(Hex));
  handlebars.register_helper("trim", Box::new(Trim));
  handlebars.register_helper("UppercaseFirstLetter", Box::new(UppercaseFirstLetter));
  handlebars.register_helper("ToLowerCase", Box::new(ToLowerCase));
  handlebars.register_helper("StartWith", Box::new(StartWith));
  handlebars.register_helper("SplitGetFirst", Box::new(SplitGetFirst));
  handlebars.register_helper("SplitGetLast", Box::new(SplitGetLast));
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

/// Returns a string slice with leading and trailing whitespace removed.
/// ```
/// # use codegenr::custom_helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({ "value": " test " }), "{{trim value}}"),
///   "test"
/// );
/// //assert_eq!(
/// //  test_helper(json!({ "value": "-test-" }), "{{trim value \"-\"}}"),
/// //  "test"
/// //);
/// ``m`
pub fn trim(v: String) -> String {
  trim_vnext(v, None)
}
pub fn trim_vnext(v: String, trimer: Option<String>) -> String {
  let trimer = get_char_trimer(trimer);

  v.trim_matches(trimer).to_string()
}

fn get_char_trimer(trimer: Option<String>) -> char {
  trimer.unwrap_or_else(|| " ".to_string()).chars().next().unwrap_or(' ')
}

handlebars_helper!(Trim: |v: String| trim(v) );

/// Returns a string in Pascal case
/// ```
/// # use codegenr::custom_helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({ "value": "tEsT" }), "{{uppercase_first_letter value}}"),
///   "TEsT"
/// );
/// ```
pub fn uppercase_first_letter(v: String) -> String {
  if v.is_empty() || !v.contains(char::is_alphanumeric) {
    return "".to_string();
  }
  let mut ve: Vec<char> = v.chars().collect();
  ve[0] = ve[0].to_uppercase().next().unwrap();
  let result: String = ve.into_iter().collect();
  result
}
handlebars_helper!(UppercaseFirstLetter: |v: String| uppercase_first_letter(v));

pub fn to_lower_case(v: String) -> String {
  v.to_lowercase()
}
handlebars_helper!(ToLowerCase: |v: String| v.to_lowercase());

//handlebars_helper!(IfEmpty: |v: String| CheckIfEmpty(v));

handlebars_helper!(IfNotEmpty: |v: String| check_if_not_empty(v));

handlebars_helper!(StartWith: |v: String| check_if_start_with(v));

/// Return the first value of a String splited by a choosen parametter
///
/// # Exemple
/// ```
/// # use codegenr::custom_helpers::*;
/// # use serde_json::json;
/// let x = "./test/lol/notme".to_string();
/// let y = "/".to_string();
/// assert_eq!(split_get_first(x, y), "test");
/// ```
pub fn split_get_first(v: String, w: String) -> String {
  for res in v.split(&w) {
    if res.is_empty() || res.contains(char::is_whitespace) || !res.contains(char::is_alphanumeric) {
      continue;
    }
    return res.to_string();
  }
  Default::default()
}
handlebars_helper!(SplitGetFirst: |v: String, w: String| split_get_first(v, w));

/// Return the value value of a String splited by a choosen parametter
///
/// # Exemple
/// ```
/// # use codegenr::custom_helpers::*;
/// let x = "test/notme/me".to_string();
/// let y = "/".to_string();
/// assert_eq!(split_get_last(x, y), "me");
/// ```
pub fn split_get_last(v: String, w: String) -> String {
  v.split(&w).last().unwrap().to_string()
}
handlebars_helper!(SplitGetLast: |v: String, w: String| split_get_last(v, w));

///
///
/// # Exemple
/// ```
/// # use codegenr::custom_helpers::*;
///
// { test: 42 }	{{trim_start test}}	42
// { test: ' 42' }	{{trim_start test}}	42
// { test: '- aa' }	{{trim_start test '-'}}	aa
// { test: 'AA' }	{{trim_start test 'A'}}	``
// { test: ' test ' }	{{trim_start test ' t'}}	est
/// ```
pub fn trim_start(v: String) -> String {
  v.trim_start().to_string()
}
handlebars_helper!(TrimStart: |v: String| trim_start(v) );

//pub fn trim_end(v: String) -> String {}
//handlebars_helper!(TrimEnd: |v: String| trim_end(v));

pub fn check_if_not_empty(v: String) -> String {
  todo!()
}

pub fn check_if_start_with(v: String) -> String {
  todo!()
}

// #[cfg(doctest)]
pub fn test_helper(json: serde_json::Value, template: &str) -> String {
  let mut h = Handlebars::new();
  register_custom_helpers(&mut h);
  h.register_template_string("test", template).expect("OK!");
  h.render("test", &json).unwrap()
}
#[cfg(test)]
mod test {
  use super::*;
  use test_case::test_case;

  #[test_case(" 42 ", "42 ")]
  fn trim_start_test(value: &str, expected: &str) {
    assert_eq!(trim_start(value.to_string()), expected.to_string());
  }

  #[test_case(" 42 ", "", "42")]
  #[test_case("-4 2-", "-", "4 2")]
  fn trim_test(value: &str, trimer: &str, expected: &str) {
    let trimer = if !trimer.is_empty() { Some(trimer.to_string()) } else { None };
    assert_eq!(trim_vnext(value.to_string(), trimer), expected.to_string());
  }
}
