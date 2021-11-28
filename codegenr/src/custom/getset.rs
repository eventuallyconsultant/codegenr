use super::handlebars_ext::HandlebarsExt;
use super::string_ext::StringExt;
use handlebars::{HelperDef, RenderError, Renderable};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap};

pub const GET_HELPER: &str = "get";
pub const SET_HELPER: &str = "set";
pub const CLEAR_HELPER: &str = "clear";
pub const WITH_GET_HELPER: &str = "with_get";
pub const WITH_SET_HELPER: &str = "with_set";

// [HandlebarsHelperSpecification("{}", "{{set 'key', 'value'}}{{get 'key'}}", "value")]
// [HandlebarsHelperSpecification("{ key: 'value' }", "{{set 'k', . }}{{#with_get 'k'}}{{key}}{{/with_get}}", "value")]
// [HandlebarsHelperSpecification("{ key: 'value' }", "{{#with_set 'key', .key }}{{get 'key'}}{{/with_set}}{{get 'key'}}", "value")]
// [HandlebarsHelperSpecification("{}", "{{set 'key', '42' }}{{get 'key'}}{{clear 'key'}}{{get 'key'}}", "42")]

/// TODO
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;

/// ```
pub struct GetHelper {
  values: RefCell<HashMap<String, Value>>,
}

impl GetHelper {
  pub fn new(values: &RefCell<HashMap<String, Value>>) -> Self {
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

    match self.values.borrow().get(&key) {
      Some(v) => Ok(v.clone().into()),
      None => Err(RenderError::new(format!("First {} param should be a string.", GET_HELPER))),
    }
  }
}

pub struct SetHelper {
  values: RefCell<HashMap<String, Value>>,
}

impl SetHelper {
  pub fn new(values: &RefCell<HashMap<String, Value>>) -> Self {
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
    h.ensure_arguments_count(2, GET_HELPER)?;

    let key = h
      .get_param_as_str(0)
      .map(ToString::to_string)
      .ok_or_else(|| RenderError::new(format!("First {} param should be a string.", GET_HELPER)))?;

    let value = h.get_param_as_json(0).ok_or_else(|| RenderError::new("Not happening."))?;

    self.values.borrow_mut().insert(key, value.clone());
    Ok(())
  }
}
