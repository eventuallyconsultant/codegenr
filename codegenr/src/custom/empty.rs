use super::handlebars_ext::HandlebarsExt;
use super::string_ext::StringExt;
use handlebars::{HelperDef, Renderable};

pub const IF_NOT_EMPTY_HELPER: &str = "if_not_empty";
pub const IF_EMPTY_HELPER: &str = "if_empty";

/// Call the template if a non empty or whitespaces string is passed as parameter
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
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
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if_not_empty}}OK{{else}}NOK{{/if_not_empty}}"),
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
    let is_empty = h.get_param_as_str(0).map(|s| s.is_empty_or_whitespaces()).unwrap_or(true);

    let temp = if !is_empty { h.template() } else { h.inverse() };
    match temp {
      Some(t) => t.render(handle, ctx, render_ctx, out),
      None => Ok(()),
    }
  }
}

/// Call the template if an empty or whitespaces string is passed as parameter
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
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
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if_empty}}OK{{else}}NOK{{/if_empty}}"),
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
    let is_empty = h.get_param_as_str(0).map(|s| s.is_empty_or_whitespaces()).unwrap_or(true);

    let temp = if is_empty { h.template() } else { h.inverse() };
    match temp {
      Some(t) => t.render(handle, ctx, render_ctx, out),
      None => Ok(()),
    }
  }
}
