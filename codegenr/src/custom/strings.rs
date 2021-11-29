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
/// assert_eq!(
///   exec_template(json!({ "value": "-test-" }), "{{trim value \"-\"}}"),
///   "test"
/// );
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

/// Returns a string with the first letter in Uppercase
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

/// Returns a string with the first letter in Lowercase
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "TEST" }), "{{lowercase_first_letter value}}"),
///   "tEST"
/// );
/// ```
pub struct LowercaseFirstLetterHelper;

impl HelperDef for LowercaseFirstLetterHelper {
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

/// Return the first part of a String splited by a definable parameter ('/' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": "test/value" }), "{{split_get_first temp}}"),
///   "test"
/// );
///  assert_eq!(
///   exec_template(json!({ "temp": "-test-123-" }), "{{split_get_first temp \"-\"}}"),
///   "test"
/// );
///
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
    h.ensure_arguments_count_min(1, SPLIT_GET_FIRST_HELPER)?;
    h.ensure_arguments_count_max(2, SPLIT_GET_FIRST_HELPER)?;

    let to_split = h.get_param_as_str_or_fail(0, SPLIT_GET_FIRST_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(handlebars::ScopedJson::Derived(Value::String(to_split.split_get_first(splitter))))
  }
}

/// Return the last value of a String splited by a definable parameter ('/' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": "test/value" }), "{{split_get_last temp}}"),
///   "value"
/// );
///  assert_eq!(
///   exec_template(json!({ "temp": "-test-123-" }), "{{split_get_last temp \"-\"}}"),
///   "123"
/// );
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

    Ok(handlebars::ScopedJson::Derived(Value::String(to_split.split_get_last(splitter))))
  }
}

/// Return a string trim only at the beggining by a definable parameter (' ' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": " test " }), "{{trim_start temp}}"),
///   "test "
/// );
/// assert_eq!(
///   exec_template(json!({ "temp": "/test/" }), "{{trim_start temp \"/\"}}"),
///   "test/"
/// );
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

/// Return a string trim only at the end by a definable parameter (' ' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": " test " }), "{{trim_end temp}}"),
///   " test"
/// );
/// assert_eq!(
///   exec_template(json!({ "temp": "/test/" }), "{{trim_end temp \"/\"}}"),
///   "/test"
/// );
/// ```
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
