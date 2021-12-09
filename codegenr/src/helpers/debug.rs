use handlebars::{HelperDef, ScopedJson};
use serde_json::{json, Value};

pub const DEBUG: &str = "debug";
pub const DEBUG_CTX: &str = "debug_ctx";

/// A debug helper that output a json representation of input parameters
/// ```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{debug 42 \"42\" plop non_existing}}"),
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
    let json = format!("{:#}", json);
    out.write("\n")?;
    out.write(&json)?;
    out.write("\n")?;
    Ok(())
  }
}

/// Does not render anything but outputs the internal handlebars contexts to the commad line
/// ```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{debug_ctx 42 \"42\" plop non_existing}}"),
///   ""
/// );
/// ```
pub struct DebugCtxHelper;

impl HelperDef for DebugCtxHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    helper: &handlebars::Helper<'reg, 'rc>,
    handlebars: &'reg handlebars::Handlebars<'reg>,
    context: &'rc handlebars::Context,
    render_context: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    out.write(&format!("helper:\n{:?}\n", helper))?;
    out.write(&format!("handlebars:\n{:?}\n", handlebars))?;
    out.write(&format!("context:\n{:?}\n", context))?;
    out.write(&format!("render_context:\n{:?}\n", render_context))?;
    Ok(())
  }
}
