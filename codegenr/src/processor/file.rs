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
      .first()
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

  fn finish(&self) -> Result<Finished, ProcessorError> {
    let buffer = self.buffer.borrow();
    if std::fs::read_to_string(&self.file_path).is_ok_and(|content| content == *buffer) {
      tracing::debug!("File content is the same, not writing it again: {}", self.file_path.display());
      return Ok(Finished::FileUnchanged);
    }
    create_file_from_path(&self.file_path)
      .and_then(|mut file| std::io::Write::write_all(&mut file, buffer.as_bytes()))
      .map_err(|e| ProcessorError::Write(self.file_path.clone(), e))?;
    Ok(Finished::FileWritten)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::filesystem::{create_dir, create_file};
  use tempfile::TempDir;

  #[test]
  pub fn start_not_existing_file_should_create_file() -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("FILE_tests")?;
    let instruction = FileInstruction::new(tmp.path().to_string_lossy().into());
    let handler = instruction.start(vec!["sub/plop.txt".into()])?;
    let (_file, should_exists_path) = create_file(tmp.path(), "sub/plop.txt")?;
    assert!(should_exists_path.exists());
    handler.handle_line("hello ...")?;
    assert!(should_exists_path.exists());
    handler.finish()?;
    let content = std::fs::read_to_string(should_exists_path)?;
    assert_eq!(content, "hello ...\n");
    Ok(())
  }

  #[test]
  pub fn finish_fails_on_a_file_it_cannot_write() -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("FILE_tests")?;
    // A folder where the file should be: `File::create` fails on every OS, a read-only folder does not on Windows.
    create_dir(tmp.path(), "taken.txt")?;
    let instruction = FileInstruction::new(tmp.path().to_string_lossy().into());
    let handler = instruction.start(vec!["taken.txt".into()])?;
    handler.handle_line("hello ...")?;
    let error = handler.finish().expect_err("A folder is in the way.");
    assert!(matches!(error, ProcessorError::Write(path, _) if path == tmp.path().join("taken.txt")));
    Ok(())
  }
}
