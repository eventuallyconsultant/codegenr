use super::*;
use glob::glob;

pub const CLEAN: &str = "CLEAN";

pub struct CleanInstruction;

impl Instruction for CleanInstruction {
  fn command_name(&self) -> &'static str {
    CLEAN
  }
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, anyhow::Error> {
    let pattern = params
      .get(0)
      .ok_or_else(|| anyhow::anyhow!("{} instruction needs one '<pattern>' parameter.", CLEAN))?;
    for path in glob(pattern)?.flatten() {
      std::fs::remove_file(path)?;
    }
    Ok(Box::new(TranscientLineHandler) as Box<dyn InstructionLineHandler>)
  }
}
