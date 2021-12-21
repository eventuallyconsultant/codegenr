use glob::PatternError;
use handlebars::Handlebars;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
  // #[error("Script Error: `{0}`.")]
  // ScriptError(#[from] handlebars::error::ScriptError),
  #[error("Pattern Error: `{0}`.")]
  PatternError(#[from] PatternError),
  #[error("Error converting PathBuf to str.")]
  PathBufError(&'static str),
  #[error("File path passed has no file stem.")]
  NoFileStem(&'static str),
  #[error("Error converting OsStr to str.")]
  OstrConvertError(&'static str),
}

pub fn handlebars_setup(handlebars: &mut Handlebars, custom_helpers_folders: Vec<String>) -> Result<(), CustomError> {
  for path in custom_helpers_folders {
    let p = Path::new(&path);
    if p.is_file() {
      handlebars_add_script(handlebars, p)?;
    } else if p.is_dir() {
      let pattern = p.join("**/*.rhai");
      let str_pattern = pattern
        .to_str()
        .ok_or_else(|| CustomError::PathBufError("Error converting PathBuf to str."))?;
      for f in glob::glob(str_pattern)?.flatten() {
        handlebars_add_script(handlebars, f.as_path())?;
      }
    }
  }
  Ok(())
}

pub fn handlebars_add_script(handlebars: &mut Handlebars, script_file: impl AsRef<Path> + Clone) -> Result<(), CustomError> {
  let name = script_file
    .as_ref()
    .file_stem()
    .ok_or_else(|| CustomError::NoFileStem("File path passed has no file stem."))?
    .to_str()
    .ok_or_else(|| CustomError::OstrConvertError("Error converting OsStr to str."))?;
  handlebars.register_script_helper_file(name, script_file.clone())?;
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use serde_json::json;

  pub fn exec_template(json: serde_json::Value, template: &str) -> String {
    let mut h = Handlebars::new();
    handlebars_setup(
      &mut h,
      vec!["./_samples/rhai/param_0_len.rhai".into(), "./_samples/rhai/concat.rhai".into()],
    )
    .expect("Could not setup handlebars.");
    h.register_template_string("test", template).expect("Could not register template.");
    h.render("test", &json).expect("Template render returned an error.")
  }

  #[test]
  fn tests() {
    assert_eq!(exec_template(json!({}), "{{param_0_len \"plop\"}}"), "4");
    assert_eq!(exec_template(json!({"a": "aa", "b": "bb"}), "{{concat a b}}"), "aabb");
  }
}
