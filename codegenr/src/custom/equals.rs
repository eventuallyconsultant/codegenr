use super::handlebars_ext::HandlebarsExt;
use super::string_ext::StringExt;
use handlebars::{HelperDef, Renderable};

pub const IF_EQUALS_HELPER: &str = "if_equals";
pub const IF_NOT_EQUALS_HELPER: &str = "if_not_equals";

/// Execute template if the first argument is equal to any other argument, otherwise execute the inverse
/// (all arguments are converted to string and case insensitive compared)
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if_equals "test" "teSt"}}OK{{else}}{{/if_equals}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({ "a": "42", "b": "42" }), r#"{{#if_equals a ./b }}OK{{else}}{{/if_equals}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if_equals "test" "NO"}}OK{{else}}NOK{{/if_equals}}"#),
///   "NOK"
/// );
/// ```
pub struct IfEqualsHelper;

impl HelperDef for IfEqualsHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count_min(2, IF_NOT_EQUALS_HELPER)?;
    let is_equal = h.get_param_as_str(0);
    let mut count = 1;
    for items in is_equal {
      let left = h.get_param_as_str(count);
      if items.to_lowercase() == left.unwrap().to_lowercase() {
        let temp = h.template();
        if let Some(t) = temp {
          t.render(handle, ctx, render_ctx, out)?
        };
        return Ok(());
      }
      count += 1
    }

    let temp = h.inverse();
    if let Some(t) = temp {
      t.render(handle, ctx, render_ctx, out)?
    };
    Ok(())
  }
}

/// Execute template if the first argument is not equal to all other arguments, otherwise execute the inverse
/// (all arguments are converted to string and case insensitive compared)
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if_not_equals 'test' 'teSt'}}{{else}}NOK{{/if_not_equals}}"#),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({ "a": "42", "b": "42" }), r#"{{#if_not_equals a ./b }}{{else}}NOK{{/if_not_equals}}"#),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if_not_equals 'test' 'NO'}}OK{{else}}NOK{{/if_not_equals}}"#),
///   "OK"
/// );
/// ```
pub struct IfNotEqualsHelper;

impl HelperDef for IfNotEqualsHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count_min(2, IF_NOT_EQUALS_HELPER)?;
    let is_equal = h.get_param_as_str(0);
    let mut count = 1;
    for items in is_equal {
      let left = h.get_param_as_str(count);
      if items.to_lowercase() == left.unwrap().to_lowercase() {
        let temp = h.inverse();
        if let Some(t) = temp {
          t.render(handle, ctx, render_ctx, out)?
        };
        return Ok(());
      }
      count += 1
    }

    let temp = h.template();
    if let Some(t) = temp {
      t.render(handle, ctx, render_ctx, out)?
    };
    Ok(())
  }
}
