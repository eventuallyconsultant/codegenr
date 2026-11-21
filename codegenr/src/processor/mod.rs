use std::{
  collections::HashMap,
  path::{Component, Path, PathBuf},
};

mod clean;
mod console;
mod file;

use clean::*;
use console::*;
use file::*;
use glob::PatternError;
use thiserror::Error;

static INSTRUCTION_LINE_REGEX: once_cell::sync::Lazy<regex::Regex> =
  once_cell::sync::Lazy::new(|| regex::Regex::new("^###[^#]*$").expect("The INSTRUCTION_LINE_REGEX regex did not compile."));

#[derive(Error, Debug)]
pub enum ProcessorError {
  #[error("Io Error: `{0}`.")]
  Io(#[from] std::io::Error),
  #[error("Fmt Error: `{0}`.")]
  Fmt(#[from] std::fmt::Error),
  #[error("Pattern Error: {0}.")]
  Pattern(#[from] PatternError),
  #[error("Instruction name not found on line {0}: `{1}`.")]
  InstructionNotFound(usize, String),
  #[error("Instruction `{0}` doest not exist. Line {1}: `{2}`.")]
  InstructionNotExisting(String, usize, String),
  #[error("Closing tag found for `{0}` instruction while it does not need it. Line {1}: `{2}`.")]
  ClosingTagFound(String, usize, String),
  #[error("Missing openning tag for `{0}` instruction. Line {1}: `{2}`.")]
  MissingOpeningTag(String, usize, String),
  #[error("Missing closing tag for `{0}` instruction opened line {1}: `{2}`.")]
  MissingClosingTag(String, usize, String),
  #[error("Could not write file `{0}`: `{1}`.")]
  Write(std::path::PathBuf, std::io::Error),
  #[error("`{0}` is outside the output folder: a `FILE` or `CLEAN` path must be relative, without `..`.")]
  PathOutsideOutput(String),
  #[error("`{0}` instruction needs one '<{1}>' parameter.")]
  InstructionParameterMissing(&'static str, &'static str),
  #[error("Error converting PathBuf to str.")]
  PathBufToStrConvert,
}

pub trait Instruction {
  fn command_name(&self) -> &'static str;
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, ProcessorError>;
  fn needs_closing(&self) -> bool {
    false
  }
}

pub trait InstructionLineHandler {
  fn handle_line(&self, line: &str) -> Result<(), ProcessorError>;
  /// Called on the closing tag, where a handler writes what it gathered: a handler never closed is never finished.
  fn finish(&self) -> Result<Finished, ProcessorError> {
    Ok(Finished::Nothing)
  }
}

/// What a handler did with what it gathered, when its instruction got closed.
#[derive(Debug)]
pub enum Finished {
  Nothing,
  FileWritten,
  /// The file already held the same content: it is not written again, so it keeps its modification date.
  FileUnchanged,
}

/// What `process` did to the files of the output folder.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ProcessReport {
  pub written_files: usize,
  pub unchanged_files: usize,
}

pub struct TranscientLineHandler;

impl InstructionLineHandler for TranscientLineHandler {
  fn handle_line(&self, _line: &str) -> Result<(), ProcessorError> {
    Ok(())
  }
}

/// The path in the output folder, or an error if it would lead out of it:
/// the paths come from the templates and the source documents, which are not trusted.
// TO_HUMAN: a symbolic link already in the output folder can still lead out of it, its target is not checked.
// Add a `canonicalize` check if an output folder may hold links.
fn path_in_output(output_folder: &str, relative_path: &str) -> Result<PathBuf, ProcessorError> {
  let stays_inside = Path::new(relative_path)
    .components()
    .all(|component| matches!(component, Component::Normal(_) | Component::CurDir));
  match stays_inside {
    true => Ok(Path::new(output_folder).join(relative_path)),
    false => Err(ProcessorError::PathOutsideOutput(relative_path.into())),
  }
}

fn get_instructions(output: String) -> HashMap<&'static str, Box<dyn Instruction>> {
  let mut hash: HashMap<&'static str, Box<dyn Instruction>> = HashMap::<_, _>::with_capacity(3);
  hash.insert(CLEAN, Box::new(CleanInstruction::new(output.clone())) as Box<dyn Instruction>);
  hash.insert(FILE, Box::new(FileInstruction::new(output)) as Box<dyn Instruction>);
  hash.insert(CONSOLE, Box::new(ConsoleInstruction) as Box<dyn Instruction>);
  hash
}

#[::tracing::instrument(level = "trace")]
pub fn process(content: &str, output: String) -> Result<ProcessReport, ProcessorError> {
  let instructions = get_instructions(output);
  let mut report = ProcessReport::default();
  // The handlers of the instructions opened and not closed yet, with their opening line.
  let mut active_handlers = HashMap::<String, (usize, &str, Box<dyn InstructionLineHandler>)>::new();

  // Numbered from 1, like the lines of the `<intermediate>.rendered.txt` file in an editor.
  for (line_number, line) in (1..).zip(content.lines()) {
    let captures = INSTRUCTION_LINE_REGEX.find(line);
    match captures {
      None => {
        for (_, _, handler) in active_handlers.values() {
          handler.handle_line(line)?;
        }
      }
      Some(_match) => {
        let net_line = line.trim_start_matches('#').trim_start();
        let is_closing = net_line.starts_with('/');
        let net_line = net_line.trim_start_matches('/').trim_start();

        let mut words = net_line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
        let instruction_name = words
          .next()
          .ok_or_else(|| ProcessorError::InstructionNotFound(line_number, line.into()))?
          .to_uppercase();

        let instruction = instructions
          .get(&instruction_name.as_ref())
          .ok_or_else(|| ProcessorError::InstructionNotExisting(instruction_name.clone(), line_number, line.into()))?;

        match (is_closing, instruction.needs_closing()) {
          (true, false) => {
            return Err(ProcessorError::ClosingTagFound(instruction_name, line_number, line.into()));
          }
          (true, true) => {
            let (_, _, handler) = active_handlers
              .remove(&instruction_name)
              .ok_or_else(|| ProcessorError::MissingOpeningTag(instruction_name, line_number, line.into()))?;
            match handler.finish()? {
              Finished::Nothing => {}
              Finished::FileWritten => report.written_files += 1,
              Finished::FileUnchanged => report.unchanged_files += 1,
            }
          }
          (false, _) => {
            if let Some((opened_line_number, opened_line, _)) = active_handlers.get(&instruction_name) {
              return Err(ProcessorError::MissingClosingTag(
                instruction_name,
                *opened_line_number,
                opened_line.to_string(),
              ));
            }
            let handler = instruction.start(words.map(Into::into).collect())?;
            if instruction.needs_closing() {
              active_handlers.insert(instruction_name, (line_number, line, handler));
            }
          }
        }
      }
    }
  }

  match active_handlers.into_iter().next() {
    Some((instruction_name, (opened_line_number, opened_line, _))) => Err(ProcessorError::MissingClosingTag(
      instruction_name,
      opened_line_number,
      opened_line.into(),
    )),
    None => Ok(report),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::TempDir;
  use test_case::test_case;

  #[test]
  fn process_reports_the_files_written_and_unchanged() -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("PROCESS_tests")?;
    let output: String = tmp.path().to_string_lossy().into();
    let rendered = "### FILE a.txt\nA\n### /FILE\n### FILE b.txt\nB\n### /FILE\n### CONSOLE\nnot a file\n### /CONSOLE\n";
    let expected = ProcessReport {
      written_files: 2,
      unchanged_files: 0,
    };
    assert_eq!(process(rendered, output.clone())?, expected);

    let only_b_changed = rendered.replace("B\n", "B changed\n");
    let expected = ProcessReport {
      written_files: 1,
      unchanged_files: 1,
    };
    assert_eq!(process(&only_b_changed, output)?, expected);
    Ok(())
  }

  #[test_case("### FILE a.txt\nhello\n### NOPE\n### /FILE\n", "Instruction `NOPE` doest not exist. Line 3: `### NOPE`." ; "error after the opening")]
  #[test_case("### FILE a.txt\nnever closed\n", "Missing closing tag for `FILE` instruction opened line 1: `### FILE a.txt`." ; "never closed")]
  #[test_case(
    "### FILE a.txt\nhello\n### FILE b.txt\n### /FILE\n",
    "Missing closing tag for `FILE` instruction opened line 1: `### FILE a.txt`." ;
    "opened twice"
  )]
  fn a_file_not_properly_closed_is_an_error_and_is_not_written(rendered: &str, expected_error: &str) -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("PROCESS_tests")?;
    let error = process(rendered, tmp.path().to_string_lossy().into()).expect_err("The rendered text is not properly closed.");
    assert_eq!(error.to_string(), expected_error);
    assert!(!tmp.path().join("a.txt").exists());
    Ok(())
  }

  #[test_case("### FILE ../escaped.txt\nhello\n### /FILE\n", "../escaped.txt" ; "file above the output")]
  #[test_case("### FILE sub/../../escaped.txt\nhello\n### /FILE\n", "sub/../../escaped.txt" ; "file above the output through a subfolder")]
  #[test_case("### CLEAN ../neighbour/*\n", "../neighbour/*" ; "clean above the output")]
  fn a_path_out_of_the_output_folder_is_refused(rendered: &str, path: &str) -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("PROCESS_tests")?;
    let neighbour_file = tmp.path().join("neighbour").join("precious.txt");
    crate::filesystem::create_file(tmp.path(), "neighbour/precious.txt")?;
    let output = tmp.path().join("output").to_string_lossy().into();

    let error = process(rendered, output).expect_err("The path leads out of the output folder.");

    let expected_error = format!("`{path}` is outside the output folder: a `FILE` or `CLEAN` path must be relative, without `..`.");
    assert_eq!(error.to_string(), expected_error);
    assert!(neighbour_file.exists());
    assert!(!tmp.path().join("escaped.txt").exists());
    Ok(())
  }

  #[test]
  fn an_absolute_path_is_refused() -> anyhow::Result<()> {
    let tmp = TempDir::with_prefix("PROCESS_tests")?;
    let absolute = tmp.path().join("escaped.txt");
    let rendered = format!("### FILE {}\nhello\n### /FILE\n", absolute.display());

    let error = process(&rendered, tmp.path().join("output").to_string_lossy().into()).expect_err("The path is absolute.");

    assert!(matches!(error, ProcessorError::PathOutsideOutput(_)));
    assert!(!absolute.exists());
    Ok(())
  }

  #[test_case("a.txt" ; "file")]
  #[test_case("./sub/a.txt" ; "subfolder")]
  #[test_case("**/*.generated.rs" ; "glob")]
  fn a_path_in_the_output_folder_is_accepted(path: &str) -> anyhow::Result<()> {
    assert_eq!(path_in_output("output", path)?, Path::new("output").join(path));
    Ok(())
  }

  #[test]
  #[ignore]
  fn process_test() -> Result<(), anyhow::Error> {
    process(
      r#"
### FILE plop.rs
test
### /FILE
### CONSOLE
Hello
#### CONSOLE
###/ console
### FILE plop2.rs
test2
### / FILE
    "#,
      ".".into(),
    )?;

    Ok(())
  }
}
