use handlebars::handlebars_helper;
use handlebars::Handlebars;

///
handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));

///
/// todo: doc test
///
///
///
handlebars_helper!(trim: |v: String| v.trim());

pub fn register_custom_helpers(handlebars: &mut Handlebars) {
  handlebars.register_helper("hex", Box::new(hex));
  handlebars.register_helper("trim", Box::new(trim));
}
