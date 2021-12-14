use super::handlebars_ext::HandlebarsExt;
use handlebars::{HelperDef, RenderError, Renderable};
use std::{
  collections::{HashMap, HashSet},
  sync::RwLock,
};

pub const DISTINCTIVE: &str = "distinctive";

/// Execute template if the first argument is equal to any other argument, otherwise execute the inverse
/// (all arguments are converted to string and case insensitive compared)
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "a": "42", "b": "42" }), r#"{{#each this}}{{@key}}/{{this}} {{/each}}"#),
///   "a/42 b/42 "
/// );
/// assert_eq!(
///   exec_template(json!({ "a": "42", "b": "42" }), r#"{{#each this}}{{#distinctive "values" this}}{{@key}}/{{this}} {{/distinctive}}{{/each}}"#),
///   "a/42 "
/// );
/// ```
#[derive(Default)]
pub struct DistinctiveHelper {
  values: RwLock<HashMap<String, HashSet<String>>>,
}

impl HelperDef for DistinctiveHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count(2, DISTINCTIVE)?;
    let key = h.get_param_as_str_or_fail(0, DISTINCTIVE)?;
    let value = h.get_param_as_str_or_fail(1, DISTINCTIVE)?;

    let mut lock = self
      .values
      .write()
      .map_err(|_| RenderError::new(format!("Could not acquire lock in `{}` helper", DISTINCTIVE)))?;
    let values_for_this_key = lock.entry(key.into()).or_default();

    let inserted = values_for_this_key.insert(value.into());
    let temp = if inserted { h.template() } else { h.inverse() };

    match temp {
      Some(t) => t.render(handle, ctx, render_ctx, out),
      None => Ok(()),
    }
  }
}
