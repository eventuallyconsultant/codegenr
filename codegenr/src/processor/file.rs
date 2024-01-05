use super::*;
use crate::filesystem::{create_file_from_path, make_path_from_root};
use std::{cell::RefCell, fmt::Write, path::PathBuf};

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
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, ProcessorError> {
    let file_path = params
      .get(0)
      .ok_or(ProcessorError::InstructionParameterMissing(FILE, "file_name"))?;
    Ok(Box::new(FileLineHandler::new(&self.output_folder, file_path)?) as Box<dyn InstructionLineHandler>)
  }
  fn needs_closing(&self) -> bool {
    true
  }
}

pub struct FileLineHandler {
  file_path: PathBuf,
  buffer: RefCell<String>,
}

impl FileLineHandler {
  fn new(output_folder: &str, write_file_path: &str) -> Result<Self, ProcessorError> {
    let file_path = make_path_from_root(output_folder, write_file_path);
    Ok(Self {
      file_path,
      buffer: RefCell::new(Default::default()),
    })
  }
}

impl InstructionLineHandler for FileLineHandler {
  fn handle_line(&self, line: &str) -> Result<(), ProcessorError> {
    let f = &mut *self.buffer.borrow_mut();
    Ok(writeln!(f, "{line}")?)
  }
}

impl Drop for FileLineHandler {
  fn drop(&mut self) {
    let buffer = self.buffer.borrow();
    if let Ok(content) = std::fs::read_to_string(&self.file_path) {
      if content == buffer.as_str() {
        tracing::warn!("File content is the same, not writing it again : {}", self.file_path.display());
        return;
      }
    }

    if let Ok(mut file) = create_file_from_path(&self.file_path).map_err(|e| {
      tracing::error!("Error creating file: {}", e);
      e
    }) {
      let _ignored = std::io::Write::write_all(&mut file, buffer.as_bytes());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::filesystem::make_path_from_root;
  use tempdir::TempDir;

  #[test]
  pub fn start_not_existing_file_should_create_file() -> anyhow::Result<()> {
    let tmp = TempDir::new("FILE_tests")?;
    let instruction = FileInstruction::new(tmp.path().to_string_lossy().into());
    let handler = instruction.start(vec!["sub/plop.txt".into()])?;
    let should_exists_path = make_path_from_root(tmp.path(), "sub/plop.txt");
    assert!(should_exists_path.exists());
    handler.handle_line("hello ...")?;
    assert!(should_exists_path.exists());
    drop(handler);
    let content = std::fs::read_to_string(should_exists_path)?;
    assert_eq!(content, "hello ...\n");
    Ok(())
  }
}
