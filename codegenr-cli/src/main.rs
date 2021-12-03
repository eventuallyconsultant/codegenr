use std::collections::HashMap;

use codegenr::{run_codegenr, Options};
use structopt::StructOpt;

//https://docs.rs/structopt/latest/structopt/#specifying-argument-types

#[derive(StructOpt, Debug)]
#[structopt(name = "codegenr")]
struct Opt {
  #[structopt(long, short, help = "Source json/yaml/openapi file.")]
  pub source: String,
  #[structopt(long, short, help = "Output folder.")]
  pub output: String,
  #[structopt(
    long,
    short,
    help = "Templates folder(s), in which only one .hbs file should have no `_` as prefix (Underscored templates are partial templates)."
  )]
  pub template: Vec<String>,
  #[structopt(
    long,
    short,
    help = "Optional path to a file where the intermediate json representation of resolved source(s) will be output."
  )]
  pub intermediate: Option<String>,
  #[structopt(long, short, help = "Path to custom helper files.")]
  pub customhelpers: Vec<String>,
  #[structopt(
    long,
    short,
    help = "Global parameters values formatted `key=value`. Values will be parsed as json or strings if the json parsing fails.",
    parse(try_from_str = parse_key_val)
  )]
  pub global_parameters: Vec<(String, serde_json::Value)>,
}

// From here: https://github.com/TeXitoi/structopt/blob/master/examples/keyvalue.rs
use std::error::Error;
/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
  T: std::str::FromStr,
  T::Err: Error + 'static,
  U: std::str::FromStr,
  U::Err: Error + 'static,
{
  let pos = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
  Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

impl From<Opt> for Options {
  fn from(opt: Opt) -> Self {
    Self {
      source: opt.source,
      output: opt.output,
      template: opt.template,
      intermediate: opt.intermediate,
      customhelpers: opt.customhelpers,
      global_parameters: opt.global_parameters.into_iter().collect(),
    }
  }
}

fn main() -> Result<(), anyhow::Error> {
  let options: Options = Opt::from_args().into();
  dbg!(options);
  // run_codegenr(options)
  Ok(())
}
