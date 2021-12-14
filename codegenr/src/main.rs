use std::collections::HashMap;

use codegenr_lib::{run_codegenr, Options};
use opt::Opt;
use structopt::StructOpt;

mod opt;

pub const CODEGENR_CONFIG_FILE: &str = "codegenr.toml";

fn main() -> Result<(), anyhow::Error> {
  let options = Opt::from_args();

  let cmd = options.cmd.unwrap_or_default();
  let options_map: HashMap<String, Options> = cmd.try_into()?;

  for (name, options) in options_map {
    if let Err(e) = run_codegenr(options) {
      println!("Error while executing the `{}` section: `{}`.", name, e);
    }
  }

  Ok(())
}
