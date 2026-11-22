use clap::Parser;
use codegenr_lib::{OptionsMap, opt::Opt, run_all_codegenr, watch::watch_codegenr};

fn main() -> Result<(), anyhow::Error> {
  let options = Opt::parse();
  tracing_subscriber::fmt().with_max_level(options.log_level()).init();

  let watch = options.watch;
  let options_map: OptionsMap = options.cmd.unwrap_or_default().try_into()?;
  match watch {
    true => watch_codegenr(options_map)?,
    false => run_all_codegenr(options_map)?,
  }
  Ok(())
}
