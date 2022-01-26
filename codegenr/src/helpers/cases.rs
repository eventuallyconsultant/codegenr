use crate::helpers::handlebars_ext::HandlebarsExt;
use crate::helpers::string_ext::StringExt;
use handlebars::HelperDef;
use serde_json::Value;

pub const UPPERCASE_HELPER: &str = "upper_case";
pub const LOWERCASE_HELPER: &str = "lower_case";
pub const UPPERCASE_FIRST_LETTER_HELPER: &str = "uppercase_first_letter";
pub const LOWERCASE_FIRST_LETTER_HELPER: &str = "lowercase_first_letter";
pub const PASCAL_CASE_HELPER: &str = "pascal_case";
pub const SNAKE_CASE_HELPER: &str = "snake_case";
pub const CAMEL_CASE_HELPER: &str = "camel_case";

/// Returns the uppercase version of the string in argument
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "tEsT" }), "{{upper_case value}}"),
///   "TEST"
/// );
/// ```
pub struct UppercaseHelper;

impl HelperDef for UppercaseHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, UPPERCASE_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, UPPERCASE_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.to_uppercase())))
  }
}

/// Returns the lowercase version of the string in argument
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "TEsT" }), "{{lower_case value}}"),
///   "test"
/// );
/// ```
pub struct LowercaseHelper;

impl HelperDef for LowercaseHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, LOWERCASE_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, LOWERCASE_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.to_lowercase())))
  }
}

/// Returns a string with the first letter in Uppercase
/// ```
/// # use codegenr_lib::helpers::*;
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
    h.ensure_arguments_count(1, UPPERCASE_FIRST_LETTER_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, UPPERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.uppercase_first_letter())))
  }
}

/// Returns a string with the first letter in Lowercase
/// ```
/// # use codegenr_lib::helpers::*;
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
    h.ensure_arguments_count(1, LOWERCASE_FIRST_LETTER_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, LOWERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.lowercase_first_letter())))
  }
}

/// Returns the pascal case version of the string
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "this should be a function name." }), "{{pascal_case value}}"),
///   "ThisShouldBeAFunctionName"
/// );
/// ```
pub struct PascalcaseHelper;

impl HelperDef for PascalcaseHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, PASCAL_CASE_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, PASCAL_CASE_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.pascal_case())))
  }
}

/// Returns the snake case version of the string
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "this should be a function name" }), "{{snake_case value}}"),
///   "this_should_be_a_function_name"
/// );
/// ```
pub struct SnakecaseHelper;

impl HelperDef for SnakecaseHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, SNAKE_CASE_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, SNAKE_CASE_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.snake_case())))
  }
}

/// Returns the camel case version of the string
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "this should be a function name." }), "{{camel_case value}}"),
///   "thisShouldBeAFunctionName"
/// );
/// ```
pub struct CamelcaseHelper;

impl HelperDef for CamelcaseHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, CAMEL_CASE_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, CAMEL_CASE_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.camel_case())))
  }
}
