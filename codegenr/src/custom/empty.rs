use super::string_ext::StringExt;
use handlebars::{HelperDef, Renderable};

/// Call the template if a non empty or whitespaces string is passed as parameter
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({}), "{{#if_not_empty \"42\"}}OK{{else}}NOK{{/if_not_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   test_helper(json!({}), "{{#if_not_empty \"  \"}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   test_helper(json!({}), "{{#if_not_empty not_existing}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{#if_not_empty plop}}OK{{else}}NOK{{/if_not_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": ""}), "{{#if_not_empty plop}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{#if_not_empty not_existing}}OK{{else}}NOK{{/if_not_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{#if_not_empty}}OK{{else}}NOK{{/if_not_empty}}"),
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
    let is_empty = if let Some(Some(s)) = h.param(0).map(|p| p.value().as_str()) {
      s.is_empty_or_whitespaces()
    } else {
      true
    };

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
///   test_helper(json!({}), "{{#if_empty \"42\"}}OK{{else}}NOK{{/if_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   test_helper(json!({}), "{{#if_empty \"  \"}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   test_helper(json!({}), "{{#if_empty not_existing}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{#if_empty plop}}OK{{else}}NOK{{/if_empty}}"),
///   "NOK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": ""}), "{{#if_empty plop}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{#if_empty not_existing}}OK{{else}}NOK{{/if_empty}}"),
///   "OK"
/// );
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{#if_empty}}OK{{else}}NOK{{/if_empty}}"),
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
    let is_empty = if let Some(Some(s)) = h.param(0).map(|p| p.value().as_str()) {
      s.is_empty_or_whitespaces()
    } else {
      true
    };

    let temp = if is_empty { h.template() } else { h.inverse() };
    match temp {
      Some(t) => t.render(handle, ctx, render_ctx, out),
      None => Ok(()),
    }
  }
}
