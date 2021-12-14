use handlebars::{HelperDef, RenderError, ScopedJson};
use serde_json::Value;

use super::handlebars_ext::HandlebarsExt;

pub const IS_OAPI3_PARAM_REQUIRED: &str = "is_oapi3_parameter_required";
pub const IS_OAPI3_PROP_REQUIRED: &str = "is_oapi3_property_required";

/// Returns true is the current context is an open api 3 required parameter.
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// let json = json!({
///   "parameters": [
///     {
///       "in": "query",
///       "name": "offset",
///       "schema": {
///         "type": "integer"
///       }
///     },
///     {
///       "in": "path",
///       "name": "limit",
///       "schema": {
///         "type": "integer"
///       },
///       "required": true
///     },
///     {
///       "in": "header",
///       "name": "jwt",
///       "schema": {
///         "type": "string"
///       },
///       "required": false
///     }
///   ]
/// });
/// assert_eq!(
///   exec_template(json, "{{#each parameters}}{{name}} is {{#if (is_oapi3_parameter_required this)}}required{{else}}not required{{/if}}\n{{/each}}"),
///   r#"offset is not required
/// limit is required
/// jwt is not required
/// "#
/// );
/// ```
pub struct IsOApi3ParamRequiredHelper;

impl HelperDef for IsOApi3ParamRequiredHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, IS_OAPI3_PARAM_REQUIRED)?;
    let json = h.get_param_as_json_or_fail(0, IS_OAPI3_PARAM_REQUIRED)?;
    let required = json["required"].as_bool().unwrap_or(false);
    Ok(ScopedJson::Derived(Value::Bool(required)))
  }
}

/// Returns true is the current context is an open api 3 required component property,
/// false otherwise
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// let json = json!({
///   "type": "object",
///   "properties": {
///     "id": {
///       "type": "integer"
///     },
///     "username": {
///       "type": "string"
///     },
///     "name": {
///       "type": "string"
///     }
///   },
///   "required": [
///     "id",
///     "username"
///   ]
/// });
/// assert_eq!(
///   exec_template(json, "{{#each properties}}{{@key}} is {{#if (is_oapi3_property_required @key ../required)}}required{{else}}not required{{/if}}\n{{/each}}"),
///   r#"id is required
/// username is required
/// name is not required
/// "#
/// );
/// ```
pub struct IsOApi3PropRequiredHelper;

impl HelperDef for IsOApi3PropRequiredHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(2, IS_OAPI3_PROP_REQUIRED)?;
    let name = h.get_param_as_json_or_fail(0, IS_OAPI3_PROP_REQUIRED)?;
    let required = h
      .get_param_as_array(1)
      .map(|required_array| required_array.iter().any(|s| s == name))
      .unwrap_or(false);
    Ok(ScopedJson::Derived(Value::Bool(required)))
  }
}
