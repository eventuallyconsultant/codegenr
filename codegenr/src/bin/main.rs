use clap::Parser;
use codegenr_lib::{OptionsMap, opt::Opt, run_all_codegenr};

fn main() -> Result<(), anyhow::Error> {
  let options = Opt::parse();
  tracing_subscriber::fmt().with_max_level(options.log_level()).init();

  let cmd = options.cmd.unwrap_or_default();
  let options_map: OptionsMap = cmd.try_into()?;
  run_all_codegenr(options_map)?;
  Ok(())
}
