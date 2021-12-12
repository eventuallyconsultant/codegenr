use crate::helpers::handlebars_ext::HandlebarsExt;
use crate::helpers::string_ext::StringExt;
use handlebars::{HelperDef, RenderError};
use serde_json::Value;

pub const JSON_HELPER: &str = "json";

/// Get the json representation of the first argument passed.
/// If a second argument is true, the json is beautyfied
///```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!(42), r#"{{json this}}"#),
///   "42"
/// );
/// assert_eq!(
///   exec_template(json!({"a": "42"}), r#"{{json this}}"#),
///   "{\"a\":\"42\"}"
/// );
/// assert_eq!(
///   exec_template(json!({"a": "42"}), r#"{{json this true}}"#),
///   "{\n  \"a\": \"42\"\n}"
/// );
/// assert_eq!(
///   exec_template(json!(42), r#"{{json (json this)}}"#),
///   "\"42\""
/// );
///```
pub struct JsonHelper;

impl HelperDef for JsonHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, JSON_HELPER)?;
    h.ensure_arguments_count_max(2, JSON_HELPER)?;
    let arg = h.get_param_as_json_or_fail(0, JSON_HELPER)?;
    let beautified = h.get_param_as_bool(1).unwrap_or_default();
    let json = if beautified { format!("{:#}", arg) } else { format!("{}", arg) };
    Ok(handlebars::ScopedJson::Derived(Value::String(json)))
  }
}
