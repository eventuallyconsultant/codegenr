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
  handlebars.register_helper("trim", Box::new(Trim));
  handlebars.register_helper("lower_case", Box::new(LowerCase));
  handlebars.register_helper("upper_case", Box::new(UpperCase));
  handlebars.register_helper("uppercase_first_letter", Box::new(UppercaseFirstLetter));
  handlebars.register_helper("split_get_first", Box::new(SplitGetFirst));
  handlebars.register_helper("split_get_last", Box::new(SplitGetLast));

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
