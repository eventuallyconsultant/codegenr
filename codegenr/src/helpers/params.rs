use super::handlebars_ext::HandlebarsExt;
use handlebars::{HelperDef, RenderError, ScopedJson};
use serde_json::Value;
use std::collections::HashMap;

pub const GLOBAL_PARAMETERS_HELPER: &str = "global_parameter";

/// Gets a value from the global parameters
/// (those key=value's would be provided as parameters of `codegenr` execution)
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// # use std::collections::HashMap;
/// let mut params = HashMap::<_,_>::new();
/// params.insert("k".to_string(), json!("v"));
///
/// assert_eq!(
///   exec_template_with_global_params(json!({}), r#"{{global_parameter "k"}}"#, params.clone()),
///   "v"
/// );
/// assert_eq!(
///   exec_template_with_global_params(json!({}), r#"{{global_parameter "non_existing"}}"#, params),
///   ""
/// );
/// ```
///
/// An error will be raise if a non existing key is asked and second parameter is true
/// ```should_panic
/// # use serde_json::json;
/// # use codegenr_lib::helpers::*;
/// exec_template_with_global_params(json!({}), r#"{{global_parameter "k" true}}"#, Default::default());
/// ```
pub struct GlobalParameterHelper {
  values: HashMap<String, Value>,
}

impl GlobalParameterHelper {
  pub fn new(values: HashMap<String, Value>) -> Self {
    Self { values }
  }
}

impl HelperDef for GlobalParameterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
    h.ensure_arguments_count_min(1, GLOBAL_PARAMETERS_HELPER)?;
    h.ensure_arguments_count_max(2, GLOBAL_PARAMETERS_HELPER)?;
    let key = h.get_param_as_str_or_fail(0, GLOBAL_PARAMETERS_HELPER)?.to_string();

    match self.values.get(&key).cloned() {
      Some(v) => Ok(ScopedJson::Derived(v)),
      None => {
        let strict_mode = h.get_param_as_bool(1).unwrap_or(false);
        if strict_mode {
          Err(RenderError::new(format!(
            "`{}`, error: Cannot find a value for key `{}`",
            GLOBAL_PARAMETERS_HELPER, key
          )))
        } else {
          Ok(ScopedJson::Derived(Default::default()))
        }
      }
    }
  }
}
