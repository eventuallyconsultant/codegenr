use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub mod handlebars_ext;
pub mod string_ext;

mod cases;
mod debug;
mod distinct;
mod equals;
mod getset;
mod math;
mod openapi3;
mod params;
mod regex;
mod strings;

pub use {self::regex::*, cases::*, debug::*, distinct::*, equals::*, getset::*, math::*, openapi3::*, params::*, strings::*};

#[derive(Error, Debug)]
pub enum HelpersError {
  #[error("regex Error: `{0}`.")]
  Regex(#[from] ::regex::Error),
}

pub fn handlebars_stateless_setup(handlebars: &mut Handlebars) {
  #[cfg(debug_assertions)]
  {
    handlebars.set_dev_mode(true);
  }
  handlebars.register_escape_fn(handlebars::no_escape);
  handlebars.register_helper(DEBUG, Box::new(DebugHelper));
  handlebars.register_helper(DEBUG_CTX, Box::new(DebugCtxHelper));
  handlebars.register_helper(IS_EMPTY_HELPER, Box::new(IsEmptyHelper));
  handlebars.register_helper(IN_HELPER, Box::new(InHelper));
  handlebars.register_helper(HEX, Box::new(Hex));
  handlebars.register_helper(TRIM_CHAR_HELPER, Box::new(TrimCharHelper));
  handlebars.register_helper(TRIM_CHAR_START_HELPER, Box::new(TrimCharStartHelper));
  handlebars.register_helper(TRIM_CHAR_END_HELPER, Box::new(TrimCharEndHelper));
  handlebars.register_helper(UPPERCASE_FIRST_LETTER_HELPER, Box::new(UppercaseFirstLetterHelper));
  handlebars.register_helper(LOWERCASE_FIRST_LETTER_HELPER, Box::new(LowercaseFirstLetterHelper));
  handlebars.register_helper(TO_ASCII_HELPER, Box::new(ToAsciiHelper));
  handlebars.register_helper(SPLIT_HELPER, Box::new(SplitHelper));
  handlebars.register_helper(START_WITH_HELPER, Box::new(StartWithHelper));
  handlebars.register_helper(END_WITH_HELPER, Box::new(EndWithHelper));
  handlebars.register_helper(CONTAINS_HELPER, Box::new(ContainsHelper));
  handlebars.register_helper(WITH_MATCHING_HELPER, Box::new(WithMatchingHelper));
  handlebars.register_helper(TRIM_BLOCK_HELPER, Box::new(TrimBlockHelper));
  handlebars.register_helper(TRIM_BLOCK_START_HELPER, Box::new(TrimBlockStartHelper));
  handlebars.register_helper(TRIM_BLOCK_END_HELPER, Box::new(TrimBlockEndHelper));
  handlebars.register_helper(ONE_LINE_HELPER, Box::new(OneLineHelper));
  handlebars.register_helper(NO_EMPTY_LINES_HELPER, Box::new(NoEmptyLinesHelper));
  handlebars.register_helper(PREFIX_LINES_HELPER, Box::new(PrefixLinesHelper));
  handlebars.register_helper(REGEX_EXTRACT_HELPER, Box::new(RegexExtractHelper));
  handlebars.register_helper(REGEX_TRANSFORM_HELPER, Box::new(RegexTransformHelper));
  //handlebars.register_helper(EACH_WITH_SORT_HELPER, Box::new(EachWithSortHelper));

  handlebars.register_helper(IS_OAPI3_PARAM_REQUIRED, Box::new(IsOApi3ParamRequiredHelper));
  handlebars.register_helper(IS_OAPI3_PROP_REQUIRED, Box::new(IsOApi3PropRequiredHelper));
}

pub fn handlebars_statefull_setup(handlebars: &mut Handlebars, global_params: HashMap<String, Value>) {
  handlebars.register_helper(DISTINCTIVE, Box::<DistinctiveHelper>::default());

  let map = Default::default();
  handlebars.register_helper(GET_HELPER, Box::new(GetHelper::new(&map)));
  handlebars.register_helper(SET_HELPER, Box::new(SetHelper::new(&map)));
  handlebars.register_helper(WITH_SET_HELPER, Box::new(WithSetHelper::new(&map)));
  handlebars.register_helper(IF_SET_HELPER, Box::new(IfSetHelper::new(&map)));
  handlebars.register_helper(CLEAR_HELPER, Box::new(ClearHelper::new(&map)));

  handlebars.register_helper(GLOBAL_PARAMETERS_HELPER, Box::new(GlobalParameterHelper::new(global_params)));
}

/// Due to an helpers name collision between handlebars_misc_helpers and this crate
/// IT'S REQUIRED to call this function *FIRST* in order to have this crate's helpers override the misc ones
pub fn handlebars_misc_setup(handlebars: &mut Handlebars) {
  handlebars_misc_helpers::register(handlebars);
}

pub fn exec_template(json: serde_json::Value, template: &str) -> String {
  let mut h = Handlebars::new();
  handlebars_misc_setup(&mut h);
  handlebars_stateless_setup(&mut h);
  handlebars_statefull_setup(&mut h, Default::default());
  h.register_template_string("test", template).expect("Could not register template.");
  h.render("test", &json).expect("Template render returned an error.")
}

pub fn exec_template_with_global_params(json: serde_json::Value, template: &str, global_params: HashMap<String, Value>) -> String {
  let mut h = Handlebars::new();
  handlebars_misc_setup(&mut h);
  handlebars_stateless_setup(&mut h);
  handlebars_statefull_setup(&mut h, global_params);
  h.register_template_string("test", template).expect("Could not register template.");
  h.render("test", &json).expect("Template render returned an error.")
}

#[cfg(test)]
mod tests {
  use super::*;
  use handlebars::RenderErrorReason;
  use test_case::test_case;

  /// The templates are not trusted: none reaches the network, nor runs jsonnet, a template language of its own.
  /// The `http_attohttpc` and `jsonnet` features of `handlebars_misc_helpers` stay off (`codegenr/Cargo.toml`).
  #[test_case("{{http_get \"http://127.0.0.1:9\"}}", "http_get" ; "http get")]
  #[test_case("{{gitignore_io \"rust\"}}", "gitignore_io" ; "gitignore io")]
  #[test_case("{{#jsonnet}}{\"a\": 1}{{/jsonnet}}", "jsonnet" ; "jsonnet")]
  fn a_template_cannot_reach_the_network_nor_run_jsonnet(template: &str, helper: &str) -> anyhow::Result<()> {
    let mut h = Handlebars::new();
    handlebars_misc_setup(&mut h);
    let error = h
      .render_template(template, &Value::Null)
      .expect_err("The helper is not registered.");
    assert!(
      matches!(error.reason(), RenderErrorReason::HelperNotFound(name) if name == helper),
      "{error}"
    );
    Ok(())
  }
}
