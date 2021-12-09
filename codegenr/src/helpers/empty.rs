use super::handlebars_ext::HandlebarsExt;
use super::string_ext::StringExt;
use handlebars::{HelperDef, Renderable};
use serde_json::Value;

pub const IF_NOT_EMPTY_HELPER: &str = "if_not_empty";
pub const IF_EMPTY_HELPER: &str = "if_empty";

/// Call the template if a non empty or whitespaces string is passed as parameter, or any other non null value
/// ```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), "{{#if_not_empty 42}}OK{{else}}NOK{{/if_not_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if_not_empty \"42\"}}OK{{else}}NOK{{/if_not_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if_not_empty \"  \"}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if_not_empty not_existing}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if_not_empty plop}}OK{{else}}NOK{{/if_not_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": ""}), "{{#if_not_empty plop}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if_not_empty not_existing}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// ```
pub struct IfNotEmptyHelper;

impl HelperDef for IfNotEmptyHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    let param0 = h.get_param_as_json_or_fail(0, IF_NOT_EMPTY_HELPER)?;
    let is_empty = is_json_empty(param0);
    let temp = if !is_empty { h.template() } else { h.inverse() };
    match temp {
      Some(t) => t.render(handle, ctx, render_ctx, out),
      None => Ok(()),
    }
  }
}

/// Call the template if an empty or whitespaces string is passed as parameter
/// ```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"a": 42}), "{{#if_empty a}}OK{{else}}NOK{{/if_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if_empty \"42\"}}OK{{else}}NOK{{/if_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if_empty \"  \"}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if_empty not_existing}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if_empty plop}}OK{{else}}NOK{{/if_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": ""}), "{{#if_empty plop}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if_empty not_existing}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// ```
pub struct IfEmptyHelper;

impl HelperDef for IfEmptyHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    let param0 = h.get_param_as_json_or_fail(0, IF_EMPTY_HELPER)?;
    let is_empty = is_json_empty(param0);
    let temp = if is_empty { h.template() } else { h.inverse() };
    match temp {
      Some(t) => t.render(handle, ctx, render_ctx, out),
      None => Ok(()),
    }
  }
}

fn is_json_empty(param0: &Value) -> bool {
  match param0 {
    Value::Null => true,
    Value::String(s) => s.is_empty_or_whitespaces(),
    _ => false,
  }
}
