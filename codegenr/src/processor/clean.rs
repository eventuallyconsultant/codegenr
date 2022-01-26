use super::*;
use glob::glob;
use std::path::Path;

pub const CLEAN: &str = "CLEAN";

pub struct CleanInstruction {
  output_folder: String,
}

impl CleanInstruction {
  pub fn new(output_folder: String) -> Self {
    Self { output_folder }
  }
}

impl Instruction for CleanInstruction {
  fn command_name(&self) -> &'static str {
    CLEAN
  }
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, ProcessorError> {
    let pattern = params.get(0).ok_or(ProcessorError::InstructionParameterMissing(CLEAN, "pattern"))?;

    let full_path_pattern = Path::new(&self.output_folder).join(pattern);
    let str_pattern = full_path_pattern.to_str().ok_or(ProcessorError::PathBufToStrConvert)?;
    for path in glob(str_pattern)?.flatten() {
      if path.is_dir() {
        std::fs::remove_dir_all(path)?
      } else if path.is_file() {
        std::fs::remove_file(path)?;
      }
    }
    Ok(Box::new(TranscientLineHandler) as Box<dyn InstructionLineHandler>)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::filesystem::{create_dir, create_file};
  use tempdir::TempDir;

  #[test]
  pub fn clean_not_existing_path_should_not_fail() -> anyhow::Result<()> {
    let instruction = CleanInstruction::new("NonExistingPath".into());
    instruction.start(vec!["plop.txt".into()])?;
    Ok(())
  }

  #[test]
  pub fn clean_a_single_file() -> anyhow::Result<()> {
    let tmp = TempDir::new("CLEAN_tests")?;
    let instruction = CleanInstruction::new(tmp.path().to_string_lossy().into());
    let (_, file_path) = create_file(tmp.path(), "plop.rs")?;
    assert!(file_path.exists());
    instruction.start(vec!["plop.rs".into()])?;
    assert!(!file_path.exists());
    Ok(())
  }

  #[test]
  pub fn clean_a_pattern() -> anyhow::Result<()> {
    let tmp = TempDir::new("CLEAN_tests")?;
    let instruction = CleanInstruction::new(tmp.path().to_string_lossy().into());
    let (_, file_path1) = create_file(tmp.path(), "plop.rs")?;
    let (_, file_path2) = create_file(tmp.path(), "sub/plop.rs")?;
    let (_, file_path3) = create_file(tmp.path(), "sub/plop.txt")?;
    instruction.start(vec!["**/*.rs".into()])?;
    assert!(!file_path1.exists());
    assert!(!file_path2.exists());
    assert!(file_path3.exists());
    Ok(())
  }

  #[test]
  pub fn clean_a_directory() -> anyhow::Result<()> {
    let tmp = TempDir::new("CLEAN_tests")?;
    let instruction = CleanInstruction::new(tmp.path().to_string_lossy().into());
    let dir_path = create_dir(tmp.path(), "directory")?;
    assert!(dir_path.exists() && dir_path.is_dir());
    instruction.start(vec!["directory".into()])?;
    assert!(!dir_path.exists());
    Ok(())
  }
}
