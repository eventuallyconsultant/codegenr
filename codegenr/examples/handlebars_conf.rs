use codegenr_lib::helpers::handlebars_setup;

fn main() {
  let mut h = handlebars::Handlebars::new();
  println!("===================================================================");
  println!("Default handlebars configuration :\n{:#?}", h);

  handlebars_setup(&mut h, Default::default());
  println!("===================================================================");
  println!("codegenr handlebars configuration :\n{:#?}", h);
}
