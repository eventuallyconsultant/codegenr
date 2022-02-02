use super::handlebars_ext::HandlebarsExt;
use handlebars::HelperDef;

pub const IN_HELPER: &str = "in";

/// Returns true if the first argument is equal to any value in the second argument array
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if (in "test" ["NO"])}}OK{{else}}NOK{{/if}}"#),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if (in 42 [1,42,3])}}OK{{else}}NOK{{/if}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"array": ["NO","NO","test"]}), r#"{{#if (in "test" array)}}OK{{else}}NOK{{/if}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#if (in "test" (str_to_json "[\"NO\",\"NOPE\"]"))}}OK{{else}}NOK{{/if}}"#),
///   "NOK"
/// );
/// ```
pub struct InHelper;

impl HelperDef for InHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(2, IN_HELPER)?;
    let value = h.get_param_as_json_or_fail(0, IN_HELPER)?;
    let array = h.get_param_as_array_or_fail(1, IN_HELPER)?;
    let contains = array.iter().any(|v| v == value);
    Ok(handlebars::ScopedJson::Derived(contains.into()))
  }
}
