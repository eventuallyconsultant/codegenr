use codegenr_lib::{run_all_codegenr, Options};
use opt::Opt;
use std::collections::HashMap;
use structopt::StructOpt;

mod opt;

pub const CODEGENR_CONFIG_FILE: &str = "codegenr.toml";

fn main() -> Result<(), anyhow::Error> {
  tracing_subscriber::fmt::init();

  let options = Opt::from_args();

  let cmd = options.cmd.unwrap_or_default();
  let options_map: HashMap<String, Options> = cmd.try_into()?;
  run_all_codegenr(options_map)?;
  Ok(())
}
