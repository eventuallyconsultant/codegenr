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
pub const IF_SET_HELPER: &str = "if_set";
pub const WITH_SET_HELPER: &str = "with_set";

/// Gets a value from the key/value store
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{set "k" "v"}}{{get "k"}}"#),
///   "v"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{set "" "v"}}{{get ""}}"#),
///   "v"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{set "k" 42}}{{get "k"}}"#),
///   "42"
/// );
/// ```
///
/// An error will be raise if a non existing key is asked
/// ```should_panic
/// # use serde_json::json;
/// # use codegenr::custom::*;
/// exec_template(json!({}), r#"{{get "plop"}}"#);
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

/// Sets a value in the key/value store
/// see [`GetHelper`] for more examples
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

/// Sets a value in the key/value store and clear it at the end of the block
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "key": "value" }), r#"{{#with_set "key" key}}{{get "key"}}{{/with_set}}"#),
///   "value"
/// );
///```
/// see [`GetHelper`] for more examples
pub struct WithSetHelper {
  values: Arc<RwLock<HashMap<String, Value>>>,
}

impl WithSetHelper {
  pub fn new(values: &Arc<RwLock<HashMap<String, Value>>>) -> Self {
    Self { values: values.clone() }
  }
}

impl HelperDef for WithSetHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count(2, WITH_SET_HELPER)?;

    let key = h
      .get_param_as_str(0)
      .map(ToString::to_string)
      .ok_or_else(|| RenderError::new(format!("First {} param should be a string.", WITH_SET_HELPER)))?;

    let value = h.get_param_as_json(1).ok_or_else(|| RenderError::new("Not happening."))?;

    let mut lock = self
      .values
      .write()
      .map_err(|_| RenderError::new(format!("Could not acquire lock in {} helper", SET_HELPER)))?;
    lock.insert(key.clone(), value.clone());
    drop(lock);

    if let Some(t) = h.template() {
      t.render(handle, ctx, render_ctx, out)?;
    }

    let mut lock = self
      .values
      .write()
      .map_err(|_| RenderError::new(format!("Could not acquire lock in {} helper", SET_HELPER)))?;
    lock.remove(&key);
    Ok(())
  }
}

/// Sets a value in the key/value store and clear it at the end of the block
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{set "k" 42}}{{#if_set "k"}}OK{{/if_set}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if_set "k"}}OK{{else}}NOK{{/if_set}}"#),
///   "NOK"
/// );
///```
/// see [`GetHelper`] for more examples
pub struct IfGetHelper {
  values: Arc<RwLock<HashMap<String, Value>>>,
}

impl IfGetHelper {
  pub fn new(values: &Arc<RwLock<HashMap<String, Value>>>) -> Self {
    Self { values: values.clone() }
  }
}

impl HelperDef for IfGetHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count(1, IF_SET_HELPER)?;

    let key = h
      .get_param_as_str(0)
      .map(ToString::to_string)
      .ok_or_else(|| RenderError::new(format!("First {} param should be a string.", IF_SET_HELPER)))?;

    let lock = self
      .values
      .read()
      .map_err(|_e| RenderError::new(format!("Could not acquire lock in {} helper", GET_HELPER)))?;

    let has_value = lock.get(&key).is_some();
    let temp = if has_value { h.template() } else { h.inverse() };
    if let Some(t) = temp {
      t.render(handle, ctx, render_ctx, out)?
    };

    Ok(())
  }
}
