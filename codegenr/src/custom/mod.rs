use handlebars::Handlebars;

pub mod handlebars_ext;
pub mod string_ext;

mod debug;
pub use debug::*;
mod empty;
pub use empty::*;
mod strings;
pub use strings::*;
mod math;
pub use math::*;
mod getset;
pub use getset::*;

pub fn handlebars_setup(handlebars: &mut Handlebars) {
  #[cfg(debug_assertions)]
  {
    handlebars.set_dev_mode(true);
  }
  handlebars.register_helper("debug", Box::new(DebugHelper));
  handlebars.register_helper("debug_ctx", Box::new(DebugCtxHelper));
  handlebars.register_helper("if_empty", Box::new(IfEmptyHelper));
  handlebars.register_helper("if_not_empty", Box::new(IfNotEmptyHelper));
  handlebars.register_helper("hex", Box::new(Hex));
  handlebars.register_helper(TRIM_HELPER, Box::new(TrimHelper));
  handlebars.register_helper(TRIM_START_HELPER, Box::new(TrimStartHelper));
  handlebars.register_helper(TRIM_END_HELPER, Box::new(TrimEndHelper));
  handlebars.register_helper(UPPERCASE_FIRST_LETTER_HELPER, Box::new(UppercaseFirstLetterHelper));
  handlebars.register_helper(LOWERCASE_FIRST_LETTER_HELPER, Box::new(LowercaseFirstLetterHelper));
  handlebars.register_helper(SPLIT_GET_FIRST_HELPER, Box::new(SplitGetFirstHelper));
  handlebars.register_helper(SPLIT_GET_LAST_HELPER, Box::new(SplitGetLastHelper));

  let map = Default::default();
  handlebars.register_helper(GET_HELPER, Box::new(GetHelper::new(&map)));
  handlebars.register_helper(SET_HELPER, Box::new(SetHelper::new(&map)));
  handlebars.register_helper(WITH_SET_HELPER, Box::new(WithSetHelper::new(&map)));
  handlebars.register_helper(IF_SET_HELPER, Box::new(IfGetHelper::new(&map)));
  handlebars.register_helper(CLEAR_HELPER, Box::new(ClearHelper::new(&map)));
}

pub fn exec_template(json: serde_json::Value, template: &str) -> String {
  let mut h = Handlebars::new();
  handlebars_setup(&mut h);
  h.register_template_string("test", template).expect("Could not register template.");
  h.render("test", &json).expect("Template render returned an error")
}
