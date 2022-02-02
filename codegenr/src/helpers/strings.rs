use crate::helpers::handlebars_ext::HandlebarsExt;
use crate::helpers::string_ext::StringExt;
use handlebars::{BlockContext, HelperDef, RenderError, Renderable, ScopedJson, StringOutput};
use serde_json::Value;

pub const SPLIT_GET_FIRST_HELPER: &str = "split_get_first";
pub const SPLIT_GET_LAST_HELPER: &str = "split_get_last";
pub const TRIM_CHAR_HELPER: &str = "trim_char";
pub const TRIM_CHAR_START_HELPER: &str = "trim_char_start";
pub const TRIM_CHAR_END_HELPER: &str = "trim_char_end";
pub const START_WITH_HELPER: &str = "start_with";
pub const WITH_MATCHING_HELPER: &str = "with_matching";
pub const EACH_WITH_SORT_HELPER: &str = "each_with_sort";
pub const TRIM_BLOCK_HELPER: &str = "trim_block";
pub const TRIM_BLOCK_START_HELPER: &str = "trim_block_start";
pub const TRIM_BLOCK_END_HELPER: &str = "trim_block_end";
pub const ONE_LINE_HELPER: &str = "one_line";
pub const NO_EMPTY_LINES_HELPER: &str = "no_empty_lines";
pub const IS_EMPTY_HELPER: &str = "is_empty";

/// Returns a string slice with leading and trailing whitespace removed.
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": " test " }), "{{trim_char value}}"),
///   "test"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "-test-" }), "{{trim_char value \"-\"}}"),
///   "test"
/// );
/// ```
pub struct TrimCharHelper;

impl HelperDef for TrimCharHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_CHAR_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_CHAR_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_CHAR_HELPER)?.to_string();
    let trimmer = h.get_param_as_str(1).map(|s| s.to_string());

    Ok(Value::String(to_trim.trim_char(trimmer)).into())
  }
}

/// Return the first part of a String splited by a definable parameter ('/' by default)
///
/// ```
/// # use codegenr_lib::helpers::*;
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
/// # use codegenr_lib::helpers::*;
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
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": " test " }), "{{trim_char_start temp}}"),
///   "test "
/// );
/// assert_eq!(
///   exec_template(json!({ "temp": "/test/" }), "{{trim_char_start temp \"/\"}}"),
///   "test/"
/// );
/// ```
pub struct TrimCharStartHelper;

impl HelperDef for TrimCharStartHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_CHAR_START_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_CHAR_START_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_CHAR_START_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());
    Ok(handlebars::ScopedJson::Derived(Value::String(to_trim.trim_start_char(splitter))))
  }
}

/// Return a string trim only at the end by a definable parameter (' ' by default)
///
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "temp": " test " }), "{{trim_char_end temp}}"),
///   " test"
/// );
/// assert_eq!(
///   exec_template(json!({ "temp": "/test/" }), "{{trim_char_end temp \"/\"}}"),
///   "/test"
/// );
/// ```
pub struct TrimCharEndHelper;

impl HelperDef for TrimCharEndHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
    h.ensure_arguments_count_min(1, TRIM_CHAR_START_HELPER)?;
    h.ensure_arguments_count_max(2, TRIM_CHAR_START_HELPER)?;

    let to_trim = h.get_param_as_str_or_fail(0, TRIM_CHAR_END_HELPER)?;
    let splitter = h.get_param_as_str(1).map(|s| s.to_string());
    Ok(handlebars::ScopedJson::Derived(Value::String(to_trim.trim_end_char(splitter))))
  }
}

/// Determines whether the beginning of the second argumentmatches the second one
///```
/// # use codegenr_lib::helpers::*;
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
/// # use codegenr_lib::helpers::*;
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
        "Arguments number for the `{}` helper must be an odd number.",
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

/// Trim start and end of a block output
/// (all arguments are converted to string and case insensitive compared)
///```
/// # use codegenr_lib::helpers::*;
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
/// # use codegenr_lib::helpers::*;
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
/// # use codegenr_lib::helpers::*;
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
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} {{/one_line}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} |do not < remove please >| {{/one_line}}"),
///   "|do not < remove please >|\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} \n {{/one_line}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}\n {{/one_line}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}\n{{/one_line}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} \r\n {{/one_line}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}\r\n{{/one_line}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} test{{/one_line}}"),
///   "test\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}} a \n z {{/one_line}}"),
///   "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a\n z{{/one_line}}"),
///   "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a\nz{{/one_line}}"),
///   "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a \r\n z{{/one_line}}"),
///   "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}a \r\n \r\n \r\nz{{/one_line}}"),
///   "az\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line 2 true \"-\"}}a \r\n \r\n \r\nz{{/one_line}}"),
///   "  a---z\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}test\r\n\r\n\r\ntest{{/one_line}}"),
///   "testtest\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line 0 \"true\"}}test\r\n\r\n\r\ntest{{/one_line}}"),
///  "testtest\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line 0 false}}test\r\n\r\n\r\ntest{{/one_line}}"),
///   "testtest"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#one_line}}{{/one_line}}"),
///   "\n"
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
      h.ensure_arguments_count_max(3, ONE_LINE_HELPER)?;
      let mut buffer = StringOutput::new();
      t.render(handle, ctx, render_ctx, &mut buffer)?;
      let s = buffer.into_string()?;
      let indent = h.get_param_as_integer(0);
      let line_break = h.get_param_as_bool(1);
      let replacer = h.get_param_as_str(2);
      out.write(&s.on_one_line(indent, line_break, replacer))?;
    };

    Ok(())
  }
}

/// Removes empty lines from the block
///```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({}), "{{#no_empty_lines}}{{/no_empty_lines}}"),
///   ""
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#no_empty_lines}} {{/no_empty_lines}}"),
///   "\n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#no_empty_lines}}a\n \t \n b {{/no_empty_lines}}"),
///   "a\n b \n"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#no_empty_lines}}\r\na\n \t \nb\r\nc\r\n\r\n{{/no_empty_lines}}"),
///   "a\nb\nc\n"
/// );
///```
pub struct NoEmptyLinesHelper;

impl HelperDef for NoEmptyLinesHelper {
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
      let content = buffer.into_string()?;

      let mut non_empty_lines_count: usize = 0;
      let mut empty_lines_count: usize = 0;
      for line in content.lines() {
        let is_empty = line.trim().is_empty();
        let (should_write_line, should_right_newline) = match (non_empty_lines_count, empty_lines_count, is_empty) {
          (0, 0, true) => (false, true),
          (_, _, true) => (false, false),
          (_, _, false) => (true, true),
        };
        if should_write_line {
          out.write(line)?;
        }
        if should_right_newline {
          out.write("\n")?;
        }
        if is_empty {
          non_empty_lines_count += 1;
        } else {
          empty_lines_count += 1;
        }
      }
    };
    Ok(())
  }
}

/// Returns true if an empty or whitespaces string is passed as parameter
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({"a": 42}), "{{#if (is_empty a)}}OK{{else}}NOK{{/if}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if (is_empty \"42\")}}OK{{else}}NOK{{/if}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if (is_empty \"  \")}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({}), "{{#if (is_empty not_existing)}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if (is_empty plop)}}OK{{else}}NOK{{/if}}"),
///   "NOK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": ""}), "{{#if (is_empty plop)}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// assert_eq!(
///   exec_template(json!({"plop": "plop"}), "{{#if (is_empty not_existing)}}OK{{else}}NOK{{/if}}"),
///   "OK"
/// );
/// ```
pub struct IsEmptyHelper;

impl HelperDef for IsEmptyHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
    let param0 = h.get_param_as_json_or_fail(0, IS_EMPTY_HELPER)?;
    let is_empty = is_json_empty(param0);
    Ok(ScopedJson::Derived(is_empty.into()))
  }
}

fn is_json_empty(param0: &Value) -> bool {
  match param0 {
    Value::Null => true,
    Value::String(s) => s.is_empty_or_whitespaces(),
    _ => false,
  }
}
