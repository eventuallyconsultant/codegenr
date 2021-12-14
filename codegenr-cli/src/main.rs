use codegenr_lib::{run_codegenr, Options};
use serde_json::Value;
use structopt::StructOpt;

const CODEGENR_CONFIG_FILE: &str = "codegenr.toml";

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
  pub templates: Vec<String>,
  #[structopt(
    long,
    short,
    help = "Optional path to a file where the intermediate json representation of resolved source(s) will be output. The resolved json will be output as <file>.resolved.json, the full text rendered result will be output as <file>.rendered.txt"
  )]
  pub intermediate: Option<String>,
  #[structopt(long, short, help = "Path to custom helper files.")]
  pub custom_helpers: Vec<String>,
  #[structopt(
    long,
    short,
    help = "Global parameters values formatted `key=value`. Values will be parsed as json or strings if the json parsing fails.",
    parse(try_from_str = parse_key_val)
  )]
  pub global_parameters: Vec<(String, serde_json::Value)>,
}

// From here: https://github.com/TeXitoi/structopt/blob/master/examples/keyvalue.rs
// Parse a single key-value pair
fn parse_key_val(s: &str) -> Result<(String, Value), anyhow::Error> {
  let pos = s
    .find('=')
    .ok_or_else(|| anyhow::anyhow!("Invalid key=value: no `=` found in `{}`.", s))?;
  let value = &s[pos + 1..];
  Ok((
    s[..pos].parse()?,
    value.parse().unwrap_or_else(|_| Value::String(value.to_string())),
  ))
}

impl From<Opt> for Options {
  fn from(opt: Opt) -> Self {
    Self {
      source: opt.source,
      output: opt.output,
      templates: opt.templates,
      intermediate: opt.intermediate,
      custom_helpers: opt.custom_helpers,
      global_parameters: opt.global_parameters.into_iter().collect(),
    }
  }
}

fn main() -> Result<(), anyhow::Error> {
  let codegen_file = std::fs::read_to_string(CODEGENR_CONFIG_FILE);
  let options = Opt::from_args_safe();
  match (codegen_file, options) {
    (Ok(_ignored_config_file), Ok(options)) => run_codegenr(options.into())?,
    (Ok(config_file), Err(_)) => {
      let opts: std::collections::HashMap<String, Options> = toml::from_str(&config_file)?;
      for (name, options) in opts {
        if let Err(e) = run_codegenr(options) {
          println!("Error while executing the `{}` section: `{}`", name, e);
        }
      }
    }
    (Err(_), Ok(options)) => run_codegenr(options.into())?,
    (Err(_), Err(clap_error)) => clap_error.exit(),
  }
  Ok(())
}
