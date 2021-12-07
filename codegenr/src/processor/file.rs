use std::{cell::RefCell, fs::File, io::Write, path::Path};

use super::*;

pub const FILE: &str = "FILE";

pub struct FileInstruction {
  output_folder: String,
}

impl FileInstruction {
  pub fn new(output_folder: String) -> Self {
    Self { output_folder }
  }
}

impl Instruction for FileInstruction {
  fn command_name(&self) -> &'static str {
    FILE
  }
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, anyhow::Error> {
    let file_path = params
      .get(0)
      .ok_or_else(|| anyhow::anyhow!("{} instruction needs one '<file_name>' parameter.", FILE))?;
    Ok(Box::new(FileLineHandler::new(file_path)?) as Box<dyn InstructionLineHandler>)
  }
}

pub struct FileLineHandler {
  file: RefCell<File>,
}

impl FileLineHandler {
  fn new(write_file_path: &str) -> Result<Self, anyhow::Error> {
    let file = File::create(write_file_path)?;
    Ok(Self { file: RefCell::new(file) })
  }
}

impl InstructionLineHandler for FileLineHandler {
  fn handle_line(&self, line: &str) -> Result<(), anyhow::Error> {
    self.file.borrow_mut().write_all(line.as_bytes())?;
    Ok(())
  }
}
