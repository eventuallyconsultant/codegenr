use crate::helpers::handlebars_ext::HandlebarsExt;
use crate::helpers::string_ext::StringExt;
use handlebars::HelperDef;
use serde_json::Value;

pub const UPPERCASE_FIRST_LETTER_HELPER: &str = "uppercase_first_letter";
pub const LOWERCASE_FIRST_LETTER_HELPER: &str = "lowercase_first_letter";

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
