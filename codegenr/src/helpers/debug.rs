use handlebars::HelperDef;
use serde_json::{json, Value};

pub const DEBUG: &str = "debug";
pub const DEBUG_CTX: &str = "debug_ctx";

/// A debug helper that output a json representation of input parameters
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{debug 42 \"42\" plop non_existing}}"),
///   r#"
/// [
///   {
///     "relative_path": "",
///     "value": 42
///   },
///   {
///     "relative_path": "",
///     "value": "42"
///   },
///   {
///     "relative_path": "plop",
///     "value": "plop"
///   },
///   {
///     "relative_path": "non_existing",
///     "value": null
///   }
/// ]
/// "#
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
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
///
/// let _result = exec_template(json!({"plop": "plop"}), "{{debug_ctx 42 \"42\" plop non_existing}}");
///
/// // Couldn't have this test pass deterministically, but the result should be close to this
/// // assert_eq!(
/// //   _result,
/// //   r#"helper:
/// // Helper { name: "debug_ctx", params: [PathAndJson { relative_path: None, value: Constant(Number(42)) }, PathAndJson { relative_path: None, value: Constant(String("42")) }, PathAndJson { relative_path: Some("plop"), value: Context(String("plop"), ["plop"]) }, PathAndJson { relative_path: Some("non_existing"), value: Missing }], hash: {}, template: None, inverse: None, block_param: None, block: false }
/// // handlebars:
/// // Handlebars { templates: {"test": Template { name: Some("test"), elements: [Expression(HelperTemplate { name: Name("debug_ctx"), params: [Literal(Number(42)), Literal(String("42")), Path(Relative(([Named("plop")], "plop"))), Path(Relative(([Named("non_existing")], "non_existing")))], hash: {}, block_param: None, template: None, inverse: None, block: false })], mapping: [TemplateMapping(1, 1)] }}, helpers: ["lower_case", "trim_end", "split_get_last", "not", "start_with", "trim_block_end", "trim", "one_line", "if_array_contains", "no_empty_lines", "gte", "with", "lookup", "debug_ctx", "with_matching", "lte", "trim_start", "set", "with_set", "regex_extract", "if", "if_empty", "if_set", "trim_block_start", "log", "trim_block", "global_parameter", "gt", "upper_case", "len", "pascal_case", "ne", "debug", "camel_case", "clear", "lowercase_first_letter", "or", "unless", "if_equals", "get", "if_not_empty", "if_not_equals", "snake_case", "split_get_first", "hex", "uppercase_first_letter", "each", "and", "raw", "eq", "lt"], decorators: ["inline"], strict_mode: false, dev_mode: true }
/// // context:
/// // Context { data: Object({"plop": String("plop")}) }
/// // render_context:
/// // RenderContext { inner: RenderContextInner { partials: {}, partial_block_stack: [], root_template: Some("test"), current_template: Some("test"), disable_eacape: false }, blocks: [BlockContext { base_path: [], base_value: None, block_params: BlockParams { data: {} }, local_variables: LocalVars { first: None, last: None, index: None, key: None, extra: {} } }], modified_context: None }
/// // "#
/// //);
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
