use codegenr_lib::helpers::*;

fn main() {
  let mut h = handlebars::Handlebars::new();
  println!("===================================================================");
  println!("Default handlebars configuration :\n{:#?}", h);

  handlebars_stateless_setup(&mut h);
  handlebars_statefull_setup(&mut h, Default::default());
  println!("===================================================================");
  println!("codegenr handlebars configuration :\n{:#?}", h);
}
