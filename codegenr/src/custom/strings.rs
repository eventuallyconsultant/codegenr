use crate::custom::handlebars_ext::HandlebarsExt;
use crate::custom::string_ext::StringExt;
use handlebars::{BlockContext, HelperDef, RenderError, Renderable, StringOutput};
use serde_json::Value;

pub const TRIM_HELPER: &str = "trim";
pub const UPPERCASE_FIRST_LETTER_HELPER: &str = "uppercase_first_letter";
pub const LOWERCASE_FIRST_LETTER_HELPER: &str = "lowercase_first_letter";
pub const SPLIT_GET_FIRST_HELPER: &str = "split_get_first";
pub const SPLIT_GET_LAST_HELPER: &str = "split_get_last";
pub const TRIM_START_HELPER: &str = "trim_start";
pub const TRIM_END_HELPER: &str = "trim_end";
pub const LOWER_CASE_HELPER: &str = "lower_case";
pub const UPPER_CASE_HELPER: &str = "upper_case";
pub const START_WITH_HELPER: &str = "start_with";
pub const WITH_MATCHING_HELPER: &str = "with_matching";
pub const IF_ARRAY_CONTAINS: &str = "if_array_contains";
pub const EACH_WITH_SORT_HELPER: &str = "each_with_sort";
pub const TRIM_BLOCK_HELPER: &str = "trim_block";
pub const TRIM_BLOCK_START_HELPER: &str = "trim_block_start";
pub const TRIM_BLOCK_END_HELPER: &str = "trim_block_end";
pub const ONE_LINE_HELPER: &str = "one_line";

/// Returns a string slice with leading and trailing whitespace removed.
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": " test " }), "{{trim value}}"),
///   "test"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "-test-" }), "{{trim value \"-\"}}"),
///   "test"
/// );
/// ```
pub struct TrimHelper;

impl HelperDef for TrimHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_HELPER)?.to_string();
    let trimmer = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(Value::String(to_trim.trim_char(trimmer)).into())
  }
}

/// Returns a string with the first letter in Uppercase
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "tEsT" }), "{{uppercase_first_letter value}}"),
///   "TEsT"
/// );
/// ```
pub struct UppercaseFirstLetterHelper;

impl HelperDef for UppercaseFirstLetterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, UPPERCASE_FIRST_LETTER_HELPER)?;
    h.ensure_arguments_count_max(1, UPPERCASE_FIRST_LETTER_HELPER)?;

    let to_uppercase = h.get_param_as_str_or_fail(0, UPPERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(
      to_uppercase.uppercase_first_letter(),
    )))
  }
}

/// Returns a string with the first letter in Lowercase
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "TEST" }), "{{lowercase_first_letter value}}"),
///   "tEST"
/// );
/// ```
pub struct LowercaseFirstLetterHelper;

impl HelperDef for LowercaseFirstLetterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, LOWERCASE_FIRST_LETTER_HELPER)?;
    h.ensure_arguments_count_max(1, LOWERCASE_FIRST_LETTER_HELPER)?;

    let to_lowercase = h.get_param_as_str_or_fail(0, LOWERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(
      to_lowercase.lowercase_first_letter(),
    )))
  }
}

/// Return the first part of a String splited by a definable parameter ('/' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": "test/value" }), "{{split_get_first temp}}"),
///   "test"
/// );
///  assert_eq!(
///   exec_template(json!({ "temp": "-test-123-" }), "{{split_get_first temp \"-\"}}"),
///   "test"
/// );
///
/// ```
pub struct SplitGetFirstHelper;

impl HelperDef for SplitGetFirstHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, SPLIT_GET_FIRST_HELPER)?;
    h.ensure_arguments_count_max(2, SPLIT_GET_FIRST_HELPER)?;

    let to_split = h.get_param_as_str_or_fail(0, SPLIT_GET_FIRST_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(handlebars::ScopedJson::Derived(Value::String(to_split.split_get_first(splitter))))
  }
}

/// Return the last value of a String splited by a definable parameter ('/' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": "test/value" }), "{{split_get_last temp}}"),
///   "value"
/// );
///  assert_eq!(
///   exec_template(json!({ "temp": "-test-123-" }), "{{split_get_last temp \"-\"}}"),
///   "123"
/// );
/// ```
pub struct SplitGetLastHelper;

impl HelperDef for SplitGetLastHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, SPLIT_GET_LAST_HELPER)?;
    h.ensure_arguments_count_max(2, SPLIT_GET_LAST_HELPER)?;

    let to_split = h.get_param_as_str_or_fail(0, SPLIT_GET_LAST_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(handlebars::ScopedJson::Derived(Value::String(to_split.split_get_last(splitter))))
  }
}

/// Return a string trim only at the beggining by a definable parameter (' ' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": " test " }), "{{trim_start temp}}"),
///   "test "
/// );
/// assert_eq!(
///   exec_template(json!({ "temp": "/test/" }), "{{trim_start temp \"/\"}}"),
///   "test/"
/// );
/// ```
pub struct TrimStartHelper;

impl HelperDef for TrimStartHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_START_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_START_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_START_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());
    Ok(handlebars::ScopedJson::Derived(Value::String(to_trim.trim_start_char(splitter))))
  }
}

/// Return a string trim only at the end by a definable parameter (' ' by default)
///
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": " test " }), "{{trim_end temp}}"),
///   " test"
/// );
/// assert_eq!(
///   exec_template(json!({ "temp": "/test/" }), "{{trim_end temp \"/\"}}"),
///   "/test"
/// );
/// ```
pub struct TrimEndHelper;

impl HelperDef for TrimEndHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_START_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_START_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_END_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());
    Ok(handlebars::ScopedJson::Derived(Value::String(to_trim.trim_end_char(splitter))))
  }
}

/// Determines whether the beginning of the second argumentmatches the second one
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"one": "test-one", "two": "one-test"}), r#"{{#start_with "test" one}}OK{{else}}{{/start_with}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"one": "test-one", "two": "one-test"}), r#"{{#start_with "test" two}}OK{{else}}NOK{{/start_with}}"#),
///   "NOK"
/// );
///```
pub struct StartWithHelper;

impl HelperDef for StartWithHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count(2, START_WITH_HELPER)?;
    let start = h.get_param_as_str_or_fail(0, START_WITH_HELPER)?;
    let with = h.get_param_as_str_or_fail(1, START_WITH_HELPER)?;

    let temp = if with.starts_with(start) { h.template() } else { h.inverse() };
    if let Some(t) = temp {
      t.render(handle, ctx, render_ctx, out)?
    };
    Ok(())
  }
}

/// Execute the inner template with the matching parameter, when matching key is equal to the first parameter
/// {{#with_matching some_value matching_key1 context1 mateching_key2 context2 ... }}
/// Render the inverse template if no matching key was found
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
///
/// assert_eq!(
///   exec_template(json!({}), r#"{{#with_matching "test" "1" "1" "2" "2"}}{{else}}NOT FOUND{{/with_matching}}"#),
///   "NOT FOUND"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#with_matching "2" "1" "01" "2" "02"}}{{this}}{{else}}NOT FOUND{{/with_matching}}"#),
///   "02"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "42" }), r#"{{#with_matching value "42" "toto"}}{{this}}{{else}}NOT FOUND{{/with_matching}}"#),
///   "toto"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "42" }), r#"{{#with_matching value "43" "toto"}}{{this}}{{else}}NOT FOUND{{/with_matching}}"#),
///   "NOT FOUND"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "42" }), r#"{{#with_matching value "42" "toto"}}{{this}}{{else}}NOT FOUND{{/with_matching}}_and_{{value}}"#),
///   "toto_and_42"
/// );
///```
pub struct WithMatchingHelper;

impl HelperDef for WithMatchingHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count_min(3, WITH_MATCHING_HELPER)?;
    let arguments_count = h.params().len();
    if arguments_count % 2 != 1 {
      return Err(RenderError::new(format!(
        "Arguments number for the '{}' helper must be an odd number.",
        WITH_MATCHING_HELPER
      )));
    }

    let key = h.get_param_as_json_or_fail(0, WITH_MATCHING_HELPER)?;

    let mut pair_position = 1;
    while pair_position < arguments_count {
      let match_key = h.get_param_as_json_or_fail(pair_position, WITH_MATCHING_HELPER)?;
      // todo: for strings, be case insensitive : value.to_lowercase() == match_key.unwrap().to_lowercase()
      if key == match_key {
        if let Some(t) = h.template() {
          let match_value = h.get_param_as_json_or_fail(pair_position + 1, WITH_MATCHING_HELPER)?;
          let mut block = BlockContext::new();
          block.set_base_value(match_value.clone());
          render_ctx.push_block(block);
          t.render(handle, ctx, render_ctx, out)?;
          render_ctx.pop_block();
        };
        return Ok(());
      }
      pair_position += 2;
    }

    if let Some(t) = h.inverse() {
      t.render(handle, ctx, render_ctx, out)?
    };
    Ok(())
  }
}

/// Write the template if the second argument is found in the array passed as first argument
/// (values are compared with string insensitive comparison)
/// (Pas completement fonctionnelle)
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// let json_array = json!({ "type": "object", "required": [ "errorMeSSage", "test" ], "properties": {"errorMessage": {"type": "string"}, "non_required_prop" : {"type" : "int"}}});
/// assert_eq!(
///   exec_template(json_array.clone(), r#"{{#if_array_contains required "errorMeSSage"}}OK{{else}}NOK{{/if_array_contains}}"#),
///   "OK"
/// );
/// //assert_eq!(
///   //exec_template(json_array.clone(), r#"{{#if_array_contains required "errormessage"}}OK{{else}}NOK{{/if_array_contains}}"#),
///   //"OK"
/// //);
/// assert_eq!(
///   exec_template(json_array.clone(), r#"{{#if_array_contains required "test"}}OK{{else}}NOK{{/if_array_contains}}"#),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json_array.clone(), r#"{{#if_array_contains required "notFound"}}OK{{else}}NOK{{/if_array_contains}}"#),
///   "NOK"
/// );
///```
pub struct IfArrayContainsHelper;

impl HelperDef for IfArrayContainsHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    h.ensure_arguments_count(2, IF_ARRAY_CONTAINS)?;
    let value = h.get_param_as_array_or_fail(0, IF_ARRAY_CONTAINS)?;
    let key = h.get_param_as_json_or_fail(1, IF_ARRAY_CONTAINS)?;

    // todo: compare case insensitive when both strings
    let is_value_found = value.iter().any(|s| s == key);
    let temp = if is_value_found { h.template() } else { h.inverse() };

    if let Some(t) = temp {
      t.render(handle, ctx, render_ctx, out)?
    };
    Ok(())
  }
}

/// Trim start and end of a block output
/// (all arguments are converted to string and case insensitive compared)
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block " "}} 1,2,3,4 {{/trim_block}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block ","}}1,2,3,4{{/trim_block}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block ","}}1,2,3,4,{{/trim_block}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block ","}},1,2,3,4,{{/trim_block}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block ","}},,1,2,3,4,,{{/trim_block}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({"a": "42", "b": "42", "c": "42"}), r#"{{#trim_block ","}}{{#each this}}{{@key}},{{/each}}{{/trim_block}}"#),
///   "a,b,c"
/// );
///```
pub struct TrimBlockHelper;

impl HelperDef for TrimBlockHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    if let Some(t) = h.template() {
      let mut buffer = StringOutput::new();
      t.render(handle, ctx, render_ctx, &mut buffer)?;
      let s = buffer.into_string()?;
      let trimmer = h.get_param_as_str(0).map(|s| s.to_string());

      out.write(&s.trim_char(trimmer))?;
    };

    Ok(())
  }
}

/// Trim start of a block output
/// (all arguments are converted to string and case insensitive compared)
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_start}} 1,2,3,4 {{/trim_block_start}}"#),
///   "1,2,3,4 "
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_start ","}}1,2,3,4{{/trim_block_start}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_start ","}}1,2,3,4,{{/trim_block_start}}"#),
///   "1,2,3,4,"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_start ","}},1,2,3,4,{{/trim_block_start}}"#),
///   "1,2,3,4,"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_start ","}},,1,2,3,4,,{{/trim_block_start}}"#),
///   "1,2,3,4,,"
/// );
/// assert_eq!(
///   exec_template(json!({"a": "42", "b": "42", "c": "42"}), r#"{{#trim_block_start ","}}{{#each this}}{{@key}},{{/each}}{{/trim_block_start}}"#),
///   "a,b,c,"
/// );
///```
pub struct TrimBlockStartHelper;

impl HelperDef for TrimBlockStartHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    if let Some(t) = h.template() {
      let mut buffer = StringOutput::new();
      t.render(handle, ctx, render_ctx, &mut buffer)?;
      let s = buffer.into_string()?;
      let trimmer = h.get_param_as_str(0).map(|s| s.to_string());

      out.write(&s.trim_start_char(trimmer))?;
    };

    Ok(())
  }
}

/// Trim end of a block output
/// (all arguments are converted to string and case insensitive compared)
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_end " "}} 1,2,3,4 {{/trim_block_end}}"#),
///   " 1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_end ","}}1,2,3,4{{/trim_block_end}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_end ","}}1,2,3,4,{{/trim_block_end}}"#),
///   "1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_end ","}},1,2,3,4,{{/trim_block_end}}"#),
///   ",1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({}), r#"{{#trim_block_end ","}},,1,2,3,4,,{{/trim_block_end}}"#),
///   ",,1,2,3,4"
/// );
/// assert_eq!(
///   exec_template(json!({"a": "42", "b": "42", "c": "42"}), r#"{{#trim_block_end ","}}{{#each this}}{{@key}},{{/each}}{{/trim_block_end}}"#),
///   "a,b,c"
/// );
///```
pub struct TrimBlockEndHelper;

impl HelperDef for TrimBlockEndHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    if let Some(t) = h.template() {
      let mut buffer = StringOutput::new();
      t.render(handle, ctx, render_ctx, &mut buffer)?;
      let s = buffer.into_string()?;
      let trimmer = h.get_param_as_str(0).map(|s| s.to_string());
      out.write(&s.trim_end_char(trimmer))?;
    };

    Ok(())
  }
}

/// Trim end of a block output
/// (all arguments are converted to string and case insensitive compared)
///```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} {{/one_line}}"),
/// "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} |do not < remove please >| {{/one_line}}"),
///  "|do not < remove please >|\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} \n {{/one_line}}"),
/// "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}\n {{/one_line}}"),
/// "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}\n{{/one_line}}"),
/// "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} \r\n {{/one_line}}"),
/// "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}\r\n{{/one_line}}"),
/// "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} test{{/one_line}}"),
/// "test\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} a \n z {{/one_line}}"),
/// "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a\n z{{/one_line}}"),
/// "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a\nz{{/one_line}}"),
/// "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a \r\n z{{/one_line}}"),
/// "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a \r\n \r\n \r\nz{{/one_line}}"),
/// "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}test\r\n\r\n\r\ntest{{/one_line}}"),
/// "testtest\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line 0 \"true\"}}test\r\n\r\n\r\ntest{{/one_line}}"),
///  "testtest\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line 0 false}}test\r\n\r\n\r\ntest{{/one_line}}"),
/// "testtest"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}{{/one_line}}"),
///  "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}   test {{/one_line}}"),
/// "test\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line 5}}test{{/one_line}}"),
/// "     test\n"
/// );
///```
pub struct OneLineHelper;

impl HelperDef for OneLineHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    handle: &'reg handlebars::Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    if let Some(t) = h.template() {
      h.ensure_arguments_count_max(2, ONE_LINE_HELPER)?;
      let mut buffer = StringOutput::new();
      t.render(handle, ctx, render_ctx, &mut buffer)?;
      let s = buffer.into_string()?;
      let indent = h.get_param_as_integer(0);
      let line_break = h.get_param_as_bool(1);
      out.write(&s.on_one_line(indent, line_break))?;
    };

    Ok(())
  }
}
// pub struct EachWithSortHelper;

// impl HelperDef for EachWithSortHelper {
//   fn call<'reg: 'rc, 'rc>(
//     &self,
//     h: &handlebars::Helper<'reg, 'rc>,
//     handle: &'reg handlebars::Handlebars<'reg>,
//     ctx: &'rc handlebars::Context,
//     render_ctx: &mut handlebars::RenderContext<'reg, 'rc>,
//     out: &mut dyn handlebars::Output,
//   ) -> handlebars::HelperResult {
//     h.ensure_arguments_count_min(1, EACH_WITH_SORT_HELPER)?;
//     h.ensure_arguments_count_min(2, EACH_WITH_SORT_HELPER)?;
//     Ok(())
//   }
// }
