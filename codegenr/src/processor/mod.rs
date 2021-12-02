use std::collections::HashMap;

mod clean;
mod console;
mod file;

use clean::*;
use console::*;
use file::*;

static INSTRUCTION_LINE_REGEX: once_cell::sync::Lazy<regex::Regex> =
  once_cell::sync::Lazy::new(|| regex::Regex::new("^###.*$").expect("The INSTRUCTION_LINE_REGEX regex did not compile."));

pub trait Instruction {
  fn command_name(&self) -> &'static str;
  fn start(&self, params: Vec<String>) -> Result<Box<dyn InstructionLineHandler>, anyhow::Error>;
}

pub trait InstructionLineHandler {
  fn handle_line(&self, line: &str) -> Result<(), anyhow::Error>;
}

pub struct TranscientLineHandler;

impl InstructionLineHandler for TranscientLineHandler {
  fn handle_line(&self, _line: &str) -> Result<(), anyhow::Error> {
    Ok(())
  }
}

fn get_instructions() -> HashMap<&'static str, Box<dyn Instruction>> {
  let mut hash: HashMap<&'static str, Box<dyn Instruction>> = HashMap::<_, _>::with_capacity(3);
  hash.insert(CLEAN, Box::new(CleanInstruction) as Box<dyn Instruction>);
  hash.insert(FILE, Box::new(FileInstruction) as Box<dyn Instruction>);
  hash.insert(CONSOLE, Box::new(ConsoleInstruction) as Box<dyn Instruction>);
  hash
}

pub fn process(content: &str) -> Result<(), anyhow::Error> {
  let instructions = get_instructions();
  let mut active_handlers: Vec<Box<dyn InstructionLineHandler>> = vec![];

  for (line_number, line) in content.lines().enumerate() {
    let captures = INSTRUCTION_LINE_REGEX.find(line);
    match captures {
      Some(_match) => {
        let mut words = line.split(' ').skip(1).map(|s| s.trim()).filter(|s| !s.is_empty());
        dbg!(&words.clone().collect::<Vec<_>>());
        let instruction_name = words
          .next()
          .ok_or_else(|| anyhow::anyhow!("Instruction name not found on line {} : '{}'", line_number, line))?
          .to_uppercase();

        let instruction = instructions
          .get(&instruction_name.as_ref())
          .ok_or_else(|| anyhow::anyhow!("Instruction {} not found.", instruction_name))?;

        let handler = instruction.start(words.map(Into::into).collect())?;

        active_handlers.push(handler);
        dbg!("OK");
      }
      None => {
        dbg!(line);
        for h in active_handlers.iter() {
          h.handle_line(line)?;
        }
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  #[ignore]
  fn process_test() -> Result<(), anyhow::Error> {
    process(
      r#"
### FILE plop.rs test.rs
test
    "#,
    )?;

    Ok(())
  }
}
