use super::handlebars_ext::HandlebarsExt;
use handlebars::{HelperDef, RenderError, Renderable};
use serde_json::Value;
use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

pub const GET_HELPER: &str = "get";
pub const SET_HELPER: &str = "set";
pub const CLEAR_HELPER: &str = "clear";
pub const IF_GET_HELPER: &str = "if_get";
pub const WITH_SET_HELPER: &str = "with_set";

// [HandlebarsHelperSpecification("{ key: 'value' }", "{{set 'k', . }}{{#with_get 'k'}}{{key}}{{/with_get}}", "value")]
// [HandlebarsHelperSpecification("{ key: 'value' }", "{{#with_set 'key', .key }}{{get 'key'}}{{/with_set}}{{get 'key'}}", "value")]
// [HandlebarsHelperSpecification("{}", "{{set 'key', '42' }}{{get 'key'}}{{clear 'key'}}{{get 'key'}}", "42")]

/// Gets a value from the key/value store
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({}), r#"{{set "k" "v"}}{{get "k"}}"#),
///   "v"
/// );
/// assert_eq!(
///   test_helper(json!({}), r#"{{set "" "v"}}{{get ""}}"#),
///   "v"
/// );
/// assert_eq!(
///   test_helper(json!({}), r#"{{set "k" 42}}{{get "k"}}"#),
///   "42"
/// );
/// ```
///
/// An error will be raise if a non existing key is asked
/// ```should_panic
/// # use serde_json::json;
/// # use codegenr::custom::*;
/// test_helper(json!({}), r#"{{get "plop"}}"#);
/// ```
pub struct GetHelper {
  values: Arc<RwLock<HashMap<String, Value>>>,
}

impl GetHelper {
  pub fn new(values: &Arc<RwLock<HashMap<String, Value>>>) -> Self {
    Self { values: values.clone() }
  }
}

impl HelperDef for GetHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, RenderError> {
    h.ensure_arguments_count(1, GET_HELPER)?;

    let key = h
      .get_param_as_str(0)
      .map(ToString::to_string)
      .ok_or_else(|| RenderError::new(format!("First {} param should be a string.", GET_HELPER)))?;

    let lock = self
      .values
      .read()
      .map_err(|_e| RenderError::new(format!("Could not acquire lock in {} helper", GET_HELPER)))?;

    match lock.get(&key) {
      Some(v) => Ok(v.clone().into()),
      None => Err(RenderError::new(format!(
        "Value is not set for key '{}' in {} helper.",
        key, GET_HELPER
      ))),
    }
  }
}

///
pub struct SetHelper {
  values: Arc<RwLock<HashMap<String, Value>>>,
}

impl SetHelper {
  pub fn new(values: &Arc<RwLock<HashMap<String, Value>>>) -> Self {
    Self { values: values.clone() }
  }
}

impl HelperDef for SetHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _handle: &'reg handlebars::Handlebars<'reg>,
    _ctx: &'rc handlebars::Context,
    _render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    _out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count(2, SET_HELPER)?;

    let key = h
      .get_param_as_str(0)
      .map(ToString::to_string)
      .ok_or_else(|| RenderError::new(format!("First {} param should be a string.", SET_HELPER)))?;

    let value = h.get_param_as_json(1).ok_or_else(|| RenderError::new("Not happening."))?;

    let mut lock = self
      .values
      .write()
      .map_err(|_| RenderError::new(format!("Could not acquire lock in {} helper", SET_HELPER)))?;

    lock.insert(key, value.clone());
    Ok(())
  }
}
