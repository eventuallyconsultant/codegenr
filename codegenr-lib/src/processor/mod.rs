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
  fn needs_closing(&self) -> bool {
    false
  }
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

fn get_instructions(output: String) -> HashMap<&'static str, Box<dyn Instruction>> {
  let mut hash: HashMap<&'static str, Box<dyn Instruction>> = HashMap::<_, _>::with_capacity(3);
  hash.insert(CLEAN, Box::new(CleanInstruction::new(output.clone())) as Box<dyn Instruction>);
  hash.insert(FILE, Box::new(FileInstruction::new(output)) as Box<dyn Instruction>);
  hash.insert(CONSOLE, Box::new(ConsoleInstruction) as Box<dyn Instruction>);
  hash
}

pub fn process(content: &str, output: String) -> Result<(), anyhow::Error> {
  let instructions = get_instructions(output);
  let mut active_handlers = HashMap::<String, Box<dyn InstructionLineHandler>>::new();

  for (line_number, line) in content.lines().enumerate() {
    let captures = INSTRUCTION_LINE_REGEX.find(line);
    match captures {
      None => {
        for (_, h) in active_handlers.iter() {
          h.handle_line(line)?;
        }
      }
      Some(_match) => {
        let net_line = line.trim_start_matches('#').trim_start();
        let is_closing = net_line.starts_with('/');
        let net_line = net_line.trim_start_matches('/').trim_start();

        let mut words = net_line.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
        let instruction_name = words
          .next()
          .ok_or_else(|| anyhow::anyhow!("Instruction name not found on line {}: `{}`.", line_number, line))?
          .to_uppercase();

        let instruction = instructions.get(&instruction_name.as_ref()).ok_or_else(|| {
          anyhow::anyhow!(
            "Instruction `{}` doest not exist. Line {}: `{}`.",
            instruction_name,
            line_number,
            line
          )
        })?;

        match (is_closing, instruction.needs_closing()) {
          (true, false) => {
            return Err(anyhow::anyhow!(
              "Closing tag found for `{}` instruction while it does not need it. Line {}: `{}`.",
              instruction_name,
              line_number,
              line
            ));
          }
          (true, true) => {
            active_handlers.remove(&instruction_name).ok_or_else(|| {
              anyhow::anyhow!(
                "Missing openning tag for `{}` instruction. Line {}: `{}`.",
                instruction_name,
                line_number,
                line
              )
            })?;
          }
          (false, _) => {
            let handler = instruction.start(words.map(Into::into).collect())?;
            if instruction.needs_closing() {
              active_handlers.insert(instruction_name, handler);
            }
          }
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
### FILE plop.rs
test
### /FILE
### CONSOLE
Hello
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