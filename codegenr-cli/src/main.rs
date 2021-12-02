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
    help = "Global parameters values formatted `key=value`. Values will be parsed as json or strings if the json parsing fails."
  )]
  pub globalparameter: Vec<String>,
}

impl From<Opt> for Options {
  fn from(opt: Opt) -> Self {
    Self {
      source: opt.source,
      output: opt.output,
      template: opt.template,
      intermediate: opt.intermediate,
      customhelpers: opt.customhelpers,
      globalparameter: opt.globalparameter,
    }
  }
}

fn main() -> Result<(), anyhow::Error> {
  let options: Options = Opt::from_args().into();
  run_codegenr(options)
}
