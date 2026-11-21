use crate::{Options, OptionsMap};
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::fs::read_to_string;

pub const CODEGENR_CONFIG_FILE: &str = "codegenr.toml";

// https://docs.rs/clap/latest/clap/_derive/index.html
#[derive(Parser, Debug)]
#[command(name = "codegenr", version, about)]
pub struct Opt {
  // todo later
  // #[arg(
  //   long,
  //   short,
  //   help = "If set, codegenr keeps running and watch for all concerned files change. And then re-run the generation.",
  //   global = true
  // )]
  // pub watch: bool,
  #[command(subcommand)]
  pub cmd: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  #[command(name = "file", about = "Executes all generations from a config file.")]
  FromFile {
    #[arg(
      long,
      help = "Path to a full codegenr configuration file. If existing, all other command line parameters are ignored",
      default_value = CODEGENR_CONFIG_FILE
    )]
    file: String,
  },
  #[command(name = "gen", about = "Executes one generation from command line parameters.")]
  FromLine {
    #[arg(long, short, help = "Source json/yaml/openapi file.")]
    source: String,
    #[arg(long, short, help = "Output folder.")]
    output: String,
    #[arg(
      long,
      short,
      num_args = 1..,
      help = "Templates folder(s), in which only one .hbs file should have no `_` as prefix (Underscored templates are partial templates)."
    )]
    templates: Vec<String>,
    #[arg(
      long,
      short,
      help = "Optional path to a file where the intermediate json representation of resolved source(s) will be output. \
        The resolved json will be output as <file>.resolved.json, the full text rendered result will be output as <file>.rendered.txt."
    )]
    intermediate: Option<String>,
    #[arg(long, short, num_args = 1.., help = "Path to custom helper files.")]
    custom_helpers: Vec<String>,
    #[arg(
      long,
      short,
      num_args = 1..,
      help = "Global parameters values formatted `key=value`. Values will be parsed as json or strings if the json parsing fails.",
      value_parser = parse_key_val
    )]
    global_parameters: Vec<(String, serde_json::Value)>,
  },
}

impl Default for Command {
  fn default() -> Self {
    Self::FromFile {
      file: CODEGENR_CONFIG_FILE.into(),
    }
  }
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

impl TryFrom<Command> for OptionsMap {
  type Error = anyhow::Error;

  fn try_from(cmd: Command) -> Result<Self, Self::Error> {
    match cmd {
      Command::FromFile { file } => {
        let config = read_to_string(&file).map_err(|e| {
          anyhow::anyhow!(
            "Unable to read `{}` file: `{}`. Did you run codegenr in the right directory ?",
            file,
            e
          )
        })?;
        let opts: OptionsMap =
          toml::from_str(&config).map_err(|e| anyhow::anyhow!("Unable to deserialize `{}` config file: `{}`.", file, e))?;
        Ok(opts)
      }
      Command::FromLine {
        source,
        output,
        templates,
        intermediate,
        custom_helpers,
        global_parameters,
      } => {
        let options = Options {
          source,
          output,
          templates,
          intermediate,
          custom_helpers,
          global_parameters: global_parameters.into_iter().collect(),
        };
        let map = OptionsMap::from_iter(std::iter::once(("command_line".into(), options)));
        Ok(map)
      }
    }
  }
}
