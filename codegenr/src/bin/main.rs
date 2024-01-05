use codegenr_lib::{opt::Opt, run_all_codegenr, OptionsMap};
use structopt::StructOpt;

fn main() -> Result<(), anyhow::Error> {
  tracing_subscriber::fmt::init();

  let options = Opt::from_args();

  let cmd = options.cmd.unwrap_or_default();
  let options_map: OptionsMap = cmd.try_into()?;
  run_all_codegenr(options_map)?;
  Ok(())
}
