use super::*;
use crate::filesystem::create_file;
use std::{cell::RefCell, fs::File, io::Write};

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
    Ok(Box::new(FileLineHandler::new(&self.output_folder, file_path)?) as Box<dyn InstructionLineHandler>)
  }
  fn needs_closing(&self) -> bool {
    true
  }
}

pub struct FileLineHandler {
  file: RefCell<File>,
}

impl FileLineHandler {
  fn new(output_folder: &str, write_file_path: &str) -> Result<Self, anyhow::Error> {
    let (file, _) = create_file(output_folder, write_file_path)?;
    Ok(Self { file: RefCell::new(file) })
  }
}

impl InstructionLineHandler for FileLineHandler {
  fn handle_line(&self, line: &str) -> Result<(), anyhow::Error> {
    self.file.borrow_mut().write_all(line.as_bytes())?;
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use std::io::Read;

  use super::*;
  use crate::filesystem::make_path_from_root;
  use tempdir::TempDir;

  #[test]
  pub fn start_not_existing_file_should_create_file() -> anyhow::Result<()> {
    const CONTENT: &str = "hello ...";
    let tmp = TempDir::new("FILE_tests")?;
    let instruction = FileInstruction::new(tmp.path().to_string_lossy().into());
    let handler = instruction.start(vec!["sub/plop.txt".into()])?;
    let should_exists_path = make_path_from_root(tmp.path(), "sub/plop.txt");
    assert!(should_exists_path.exists());
    handler.handle_line(CONTENT)?;
    assert!(should_exists_path.exists());
    drop(handler);
    let mut content = String::new();
    File::open(should_exists_path)?.read_to_string(&mut content)?;
    assert_eq!(content, CONTENT);
    Ok(())
  }
}
