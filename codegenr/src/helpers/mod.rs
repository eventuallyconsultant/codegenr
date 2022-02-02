use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub mod handlebars_ext;
pub mod string_ext;

mod cases;
mod debug;
mod distinct;
mod empty;
mod equals;
mod getset;
mod math;
mod openapi3;
mod params;
mod regex;
mod strings;

pub use {self::regex::*, cases::*, debug::*, distinct::*, empty::*, equals::*, getset::*, math::*, openapi3::*, params::*, strings::*};

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
  handlebars.register_helper(IF_EMPTY_HELPER, Box::new(IfEmptyHelper));
  handlebars.register_helper(IF_NOT_EMPTY_HELPER, Box::new(IfNotEmptyHelper));
  handlebars.register_helper(IN_HELPER, Box::new(InHelper));
  handlebars.register_helper(HEX, Box::new(Hex));
  handlebars.register_helper(TRIM_CHAR_HELPER, Box::new(TrimCharHelper));
  handlebars.register_helper(TRIM_CHAR_START_HELPER, Box::new(TrimCharStartHelper));
  handlebars.register_helper(TRIM_CHAR_END_HELPER, Box::new(TrimCharEndHelper));
  handlebars.register_helper(UPPERCASE_FIRST_LETTER_HELPER, Box::new(UppercaseFirstLetterHelper));
  handlebars.register_helper(LOWERCASE_FIRST_LETTER_HELPER, Box::new(LowercaseFirstLetterHelper));
  handlebars.register_helper(SPLIT_GET_FIRST_HELPER, Box::new(SplitGetFirstHelper));
  handlebars.register_helper(SPLIT_GET_LAST_HELPER, Box::new(SplitGetLastHelper));
  handlebars.register_helper(START_WITH_HELPER, Box::new(StartWithHelper));
  handlebars.register_helper(WITH_MATCHING_HELPER, Box::new(WithMatchingHelper));
  handlebars.register_helper(TRIM_BLOCK_HELPER, Box::new(TrimBlockHelper));
  handlebars.register_helper(TRIM_BLOCK_START_HELPER, Box::new(TrimBlockStartHelper));
  handlebars.register_helper(TRIM_BLOCK_END_HELPER, Box::new(TrimBlockEndHelper));
  handlebars.register_helper(ONE_LINE_HELPER, Box::new(OneLineHelper));
  handlebars.register_helper(NO_EMPTY_LINES_HELPER, Box::new(NoEmptyLinesHelper));
  handlebars.register_helper(REGEX_EXTRACT_HELPER, Box::new(RegexExtractHelper));
  handlebars.register_helper(REGEX_TRANSFORM_HELPER, Box::new(RegexTransformHelper));
  //handlebars.register_helper(EACH_WITH_SORT_HELPER, Box::new(EachWithSortHelper));

  handlebars.register_helper(IS_OAPI3_PARAM_REQUIRED, Box::new(IsOApi3ParamRequiredHelper));
  handlebars.register_helper(IS_OAPI3_PROP_REQUIRED, Box::new(IsOApi3PropRequiredHelper));
}

pub fn handlebars_statefull_setup(handlebars: &mut Handlebars, global_params: HashMap<String, Value>) {
  handlebars.register_helper(DISTINCTIVE, Box::new(DistinctiveHelper::default()));

  let map = Default::default();
  handlebars.register_helper(GET_HELPER, Box::new(GetHelper::new(&map)));
  handlebars.register_helper(SET_HELPER, Box::new(SetHelper::new(&map)));
  handlebars.register_helper(WITH_SET_HELPER, Box::new(WithSetHelper::new(&map)));
  handlebars.register_helper(IF_SET_HELPER, Box::new(IfSetHelper::new(&map)));
  handlebars.register_helper(CLEAR_HELPER, Box::new(ClearHelper::new(&map)));

  handlebars.register_helper(GLOBAL_PARAMETERS_HELPER, Box::new(GlobalParameterHelper::new(global_params)));
}

pub fn handlebars_misc_setup(handlebars: &mut Handlebars) {
  handlebars_misc_helpers::register(handlebars);
}

pub fn exec_template(json: serde_json::Value, template: &str) -> String {
  let mut h = Handlebars::new();
  handlebars_stateless_setup(&mut h);
  handlebars_statefull_setup(&mut h, Default::default());
  handlebars_misc_setup(&mut h);
  h.register_template_string("test", template).expect("Could not register template.");
  h.render("test", &json).expect("Template render returned an error.")
}

pub fn exec_template_with_global_params(json: serde_json::Value, template: &str, global_params: HashMap<String, Value>) -> String {
  let mut h = Handlebars::new();
  handlebars_stateless_setup(&mut h);
  handlebars_statefull_setup(&mut h, global_params);
  handlebars_misc_setup(&mut h);
  h.register_template_string("test", template).expect("Could not register template.");
  h.render("test", &json).expect("Template render returned an error.")
}
