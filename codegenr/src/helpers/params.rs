use super::handlebars_ext::HandlebarsExt;
use handlebars::{HelperDef, RenderError};
use serde_json::Value;
use std::collections::HashMap;

pub const GLOBAL_PARAMETERS_HELPER: &str = "global_parameter";

/// Gets a value from the global parameters
/// (those key/value's would be provided as parameters of `codegenr` execution)
/// ```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// # use std::collections::HashMap;
/// let mut params = HashMap::<_,_>::new();
/// params.insert("key".to_string(), json!("v"));
///
/// assert_eq!(
///   exec_template_with_global_params(json!({}), r#"{{global_parameter "k"}}"#, params),
///   "v"
/// );
/// ```
///
/// An error will be raise if a non existing key is asked
/// ```should_panic
/// # use serde_json::json;
/// # use codegenr::helpers::*;
/// exec_template_with_global_params(json!({}), r#"{{global_parameter "k"}}"#, Default::default());
/// ```
pub struct GlobalparameterHelper {
  values: HashMap<String, Value>,
}

impl GlobalparameterHelper {
  pub fn new(values: HashMap<String, Value>) -> Self {
    Self { values }
  }
}

impl HelperDef for GlobalparameterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, RenderError> {
    h.ensure_arguments_count(1, GLOBAL_PARAMETERS_HELPER)?;
    let key = h.get_param_as_str_or_fail(0, GLOBAL_PARAMETERS_HELPER)?.to_string();
    let value = self.values.get(&key);
    match value {
      None => anyhow::private::Err(RenderError::new(format!(
        "{}, error: The key or the associated value to itself doesn't exist helper.",
        GLOBAL_PARAMETERS_HELPER
      ))),
      Some(_) => Ok(handlebars::ScopedJson::Derived(value.unwrap().clone())),
    }

    //Ok(handlebars::ScopedJson::Derived(value))
  }
}
