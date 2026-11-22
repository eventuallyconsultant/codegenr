use clap::Parser;
use codegenr_lib::{OptionsMap, opt::Opt, run_all_codegenr, watch::watch_codegenr};

fn main() -> Result<(), anyhow::Error> {
  let options = Opt::parse();
  tracing_subscriber::fmt().with_max_level(options.log_level()).init();

  let command = options.cmd.unwrap_or_default();
  match options.watch {
    true => watch_codegenr(command)?,
    false => run_all_codegenr(OptionsMap::try_from(command)?)?,
  }
  Ok(())
}
