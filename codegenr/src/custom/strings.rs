use crate::custom::handlebars_ext::HandlebarsExt;
use crate::custom::string_ext::StringExt;
use handlebars::{handlebars_helper, HelperDef};
use serde_json::Value;

pub const TRIM_HELPER: &str = "trim";
pub const UPPERCASE_FIRST_LETTER_HELPER: &str = "uppercase_first_letter";
pub const LOWERCASE_FIRST_LETTER_HELPER: &str = "lowercase_first_letter";
pub const SPLIT_GET_FIRST_HELPER: &str = "split_get_first";
pub const SPLIT_GET_LAST_HELPER: &str = "split_get_last";
pub const TRIM_START_HELPER: &str = "trim_start";
pub const TRIM_END_HELPER: &str = "trim_end";
pub const LOWER_CASE_HELPER: &str = "lower_case";
pub const UPPER_CASE_HELPER: &str = "upper_case";

/// Returns a string slice with leading and trailing whitespace removed.
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": " test " }), "{{trim value}}"),
///   "test"
/// );
/// //assert_eq!(
/// //  exec_template(json!({ "value": "-test-" }), "{{trim value \"-\"}}"),
/// //  "test"
/// //);
/// ```
pub struct TrimHelper;

impl HelperDef for TrimHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_HELPER)?.to_string();
    let trimmer = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(Value::String(to_trim.trim_char(trimmer)).into())
  }
}

/// Returns a string in Pascal case
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "tEsT" }), "{{uppercase_first_letter value}}"),
///   "TEsT"
/// );
/// ```
pub struct UppercaseFirstLetterHelper;

impl HelperDef for UppercaseFirstLetterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, UPPERCASE_FIRST_LETTER_HELPER)?;
    h.ensure_arguments_count_max(1, UPPERCASE_FIRST_LETTER_HELPER)?;

    let to_uppercase = h.get_param_as_str_or_fail(0, UPPERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(
      to_uppercase.uppercase_first_letter(),
    )))
  }
}

pub struct LowerCaseFirstLetterHelper;

impl HelperDef for LowerCaseFirstLetterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, LOWERCASE_FIRST_LETTER_HELPER)?;
    h.ensure_arguments_count_max(1, LOWERCASE_FIRST_LETTER_HELPER)?;

    let to_lowercase = h.get_param_as_str_or_fail(0, LOWERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(
      to_lowercase.lowercase_first_letter(),
    )))
  }
}

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
pub struct SplitGetFirstHelper;

impl HelperDef for SplitGetFirstHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(2, SPLIT_GET_FIRST_HELPER)?;
    h.ensure_arguments_count_max(2, SPLIT_GET_FIRST_HELPER)?;

    let to_split = h.get_param_as_str_or_fail(0, SPLIT_GET_FIRST_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(handlebars::ScopedJson::Derived(Value::String(to_split.split_get_first(splitter))))
  }
}

/// Return the value value of a String splited by a choosen parametter
///
/// # Exemple
/// ```
/// # use codegenr::custom::*;
/// let x = "test/notme/me".to_string();
/// let y = "/".to_string();
/// assert_eq!(split_get_last(x, y), "me");
/// ```
pub struct SplitGetLastHelper;

impl HelperDef for SplitGetLastHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, SPLIT_GET_LAST_HELPER)?;
    h.ensure_arguments_count_max(2, SPLIT_GET_LAST_HELPER)?;

    let to_split = h.get_param_as_str_or_fail(0, SPLIT_GET_LAST_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(handlebars::ScopedJson::Derived(Value::String(to_split.split_get_first(splitter))))
  }
}

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
pub struct TrimStartHelper;

impl HelperDef for TrimStartHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_START_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_START_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_START_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());
    Ok(handlebars::ScopedJson::Derived(Value::String(to_trim.trim_start_char(splitter))))
  }
}

pub struct TrimEndHelper;

impl HelperDef for TrimEndHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_START_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_START_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_END_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());
    Ok(handlebars::ScopedJson::Derived(Value::String(to_trim.trim_end_char(splitter))))
  }
}
