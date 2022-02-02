use super::handlebars_ext::HandlebarsExt;
use super::string_ext::StringExt;
use handlebars::{HelperDef, RenderError, ScopedJson};
use serde_json::Value;

pub const IS_EMPTY_HELPER: &str = "is_empty";

/// Returns true if an empty or whitespaces string is passed as parameter
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"a": 42}), "{{#if (is_empty a)}}OK{{else}}NOK{{/if}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if (is_empty \"42\")}}OK{{else}}NOK{{/if}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if (is_empty \"  \")}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if (is_empty not_existing)}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if (is_empty plop)}}OK{{else}}NOK{{/if}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": ""}), "{{#if (is_empty plop)}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if (is_empty not_existing)}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// ```
pub struct IsEmptyHelper;

impl HelperDef for IsEmptyHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
    let param0 = h.get_param_as_json_or_fail(0, IS_EMPTY_HELPER)?;
    let is_empty = is_json_empty(param0);
    Ok(ScopedJson::Derived(is_empty.into()))
  }
}

fn is_json_empty(param0: &Value) -> bool {
  match param0 {
    Value::Null => true,
    Value::String(s) => s.is_empty_or_whitespaces(),
    _ => false,
  }
}
