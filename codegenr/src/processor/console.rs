use super::*;

pub const CONSOLE: &str = "CONSOLE";

pub struct ConsoleInstruction;

impl Instruction for ConsoleInstruction {
  fn command_name(&self) -> &'static str {
    CONSOLE
  }
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, anyhow::Error> {
    Ok(Box::new(ConsoleLineHandler) as Box<dyn InstructionLineHandler>)
  }
}

pub struct ConsoleLineHandler;

impl InstructionLineHandler for ConsoleLineHandler {
  fn handle_line(&self, line: &str) -> Result<(), anyhow::Error> {
    println!("{}", line);
    Ok(())
  }
}
