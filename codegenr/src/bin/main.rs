use codegenr_lib::{opt::Opt, run_all_codegenr, OptionsMap};
use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
  tracing_subscriber::fmt::init();

  let options = Opt::parse();

  let cmd = options.cmd.unwrap_or_default();
  let options_map: OptionsMap = cmd.try_into()?;
  run_all_codegenr(options_map)?;
  Ok(())
}
