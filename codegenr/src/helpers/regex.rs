use crate::helpers::handlebars_ext::HandlebarsExt;
use crate::helpers::string_ext::StringExt;
use handlebars::{HelperDef, RenderError};
use serde_json::Value;

pub const REGEX_EXTRACT_HELPER: &str = "regex_extract";
pub const REGEX_TRANSFORM_HELPER: &str = "regex_transform";

/// Extract and transform a list of values with regex.
///```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"test": "/user/{username}"}), r#"{{regex_extract test "\\{([^}]*)}" "$1"}}"#),
///   "username"
/// );
/// assert_eq!(
///   exec_template(json!({"test": "/user/{username}/{id}"}), r#"{{regex_extract test "\\{([^}]*)}" "$1"}}"#),
///   "username, id"
/// );
/// assert_eq!(
///   exec_template(json!({"test": "/user/{username}/{id}"}), r#"{{regex_extract test "\\{([^}]*)}" "<$1>" "|" }}"#),
///   "<username>|<id>"
/// );
///```
pub struct RegexExtractHelper;

impl HelperDef for RegexExtractHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    let arg = h.get_param_as_str_or_fail(0, REGEX_EXTRACT_HELPER)?;
    let regex_pattern = h.get_param_as_str_or_fail(1, REGEX_EXTRACT_HELPER)?;
    let regex_replacer = h.get_param_as_str_or_fail(2, REGEX_EXTRACT_HELPER)?;
    let separator = h.get_param_as_str(3);
    let result = arg
      .regex_extract(regex_pattern, Some(regex_replacer), separator)
      .map_err(|e| RenderError::new(format!("{} error: `{}`.", REGEX_EXTRACT_HELPER, e)))?;
    Ok(handlebars::ScopedJson::Derived(Value::String(result)))
  }
}

/// Extract and transform a list of values.
///```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"test": "/user/{username}"}), r#"{{regex_transform test "\\{([^}]*)}" "$1"}}"#),
///   "/user/username"
/// );
/// assert_eq!(
///   exec_template(json!({"test": "/user/{username}/{id}"}), r#"{{regex_transform test "\\{([^}]*)}" "$1"}}"#),
///   "/user/username/id"
/// );
/// assert_eq!(
///   exec_template(json!({"test": "/user/{username}/{id}"}), r#"{{regex_transform test "\\{([^}]*)}" "<$1>" "|" }}"#),
///   "/user/<username>/<id>"
/// );
///```
pub struct RegexTransformHelper;

impl HelperDef for RegexTransformHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    let arg = h.get_param_as_str_or_fail(0, REGEX_TRANSFORM_HELPER)?;
    let regex_pattern = h.get_param_as_str_or_fail(1, REGEX_TRANSFORM_HELPER)?;
    let regex_replacer = h.get_param_as_str_or_fail(2, REGEX_TRANSFORM_HELPER)?;
    let result = arg
      .regex_transform(regex_pattern, regex_replacer)
      .map_err(|e| RenderError::new(format!("{} error: `{}`.", REGEX_TRANSFORM_HELPER, e)))?;
    Ok(handlebars::ScopedJson::Derived(Value::String(result)))
  }
}
