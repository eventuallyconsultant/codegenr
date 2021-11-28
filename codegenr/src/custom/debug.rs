use handlebars::{HelperDef, ScopedJson};
use serde_json::{json, Value};

/// A debug helper that output a json representation of input parameters
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{debug 42 \"42\" plop non_existing}}"),
///   r#"[{"relative_path":"","value":42},{"relative_path":"","value":"42"},{"relative_path":"plop","value":"plop"},{"relative_path":"non_existing","value":null}]"#
/// );
/// ```
pub struct DebugHelper;

impl HelperDef for DebugHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _r: &'reg handlebars::Handlebars<'reg>,
    _ctx: &'rc handlebars::Context,
    _rc: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    let params: Vec<_> = h
      .params()
      .iter()
      .map(|pj| {
        json! ({
          "relative_path": pj.relative_path().map(|s|s.to_owned()).unwrap_or_default(),
          "value": pj.value(),
        })
      })
      .collect();
    let json = Value::Array(params);
    let json = format!("{}", json);
    println!("{}", json);
    out.write(&json)?;
    Ok(())
  }
}

/// Does not render anything but outputs the internal handlebars contexts to the commad line
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(json!({"plop": "plop"}), "{{debug_ctx 42 \"42\" plop non_existing}}"),
///   ""
/// );
/// ```
pub struct DebugCtxHelper;

impl HelperDef for DebugCtxHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    helper: &handlebars::Helper<'reg, 'rc>,
    handlebars: &'reg handlebars::Handlebars<'reg>,
    context: &'rc handlebars::Context,
    render_context: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    println!("helper:\n{:?}", helper);
    println!("handlebars:\n{:?}", handlebars);
    println!("context:\n{:?}", context);
    println!("render_context:\n{:?}", render_context);
    Ok(ScopedJson::Derived(Value::Null))
  }
}
