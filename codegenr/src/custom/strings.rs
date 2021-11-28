use crate::custom::string_ext::StringExt;
use handlebars::handlebars_helper;

/// Returns a string slice with leading and trailing whitespace removed.
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({ "value": " test " }), "{{trim value}}"),
///   "test"
/// );
/// //assert_eq!(
/// //  test_helper(json!({ "value": "-test-" }), "{{trim value \"-\"}}"),
/// //  "test"
/// //);
/// ```
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
/// # use codegenr::custom::*;
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

pub fn lower_case(v: String) -> String {
  v.to_lowercase()
}
handlebars_helper!(LowerCase: |v: String| lower_case(v));

pub fn upper_case(v: String) -> String {
  v.to_uppercase()
}
handlebars_helper!(UpperCase: |v: String| upper_case(v));

/// Return the first value of a String splited by a choosen parametter
///
/// # Exemple
/// ```
/// # use codegenr::custom::*;
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
/// # use codegenr::custom::*;
/// let x = "test/notme/me".to_string();
/// let y = "/".to_string();
/// assert_eq!(split_get_last(x, y), "me");
/// ```
pub fn split_get_last(v: String, w: String) -> String {
  v.split_get_last(Some(w))
}
handlebars_helper!(SplitGetLast: |v: String, w: String| split_get_last(v, w));

///
///
/// # Exemple
/// ```
/// # use codegenr::custom::*;
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
