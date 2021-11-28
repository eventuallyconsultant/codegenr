use handlebars::{Helper, HelperResult, RenderError};
use serde_json::Value;

pub trait HandlebarsExt {
  fn ensure_arguments_count(&self, count: usize, helper_name: &str) -> HelperResult;
  fn ensure_arguments_count_max(&self, count: usize, helper_name: &str) -> HelperResult;
  fn ensure_arguments_count_min(&self, count: usize, helper_name: &str) -> HelperResult;
  fn get_param_as_str(&self, index: usize) -> Option<&str>;
  fn get_param_as_json(&self, index: usize) -> Option<&Value>;
}

impl<'reg, 'rc> HandlebarsExt for Helper<'reg, 'rc> {
  fn ensure_arguments_count(&self, count: usize, helper_name: &str) -> HelperResult {
    let len = self.params().len();
    if len != count {
      Err(RenderError::new(format!(
        "{} helper needs exactly {} arguments.",
        helper_name, count
      )))
    } else {
      Ok(())
    }
  }

  fn ensure_arguments_count_max(&self, count: usize, helper_name: &str) -> HelperResult {
    let len = self.params().len();
    if len > count {
      Err(RenderError::new(format!(
        "{} helper needs at most {} arguments.",
        helper_name, count
      )))
    } else {
      Ok(())
    }
  }

  fn ensure_arguments_count_min(&self, count: usize, helper_name: &str) -> HelperResult {
    let len = self.params().len();
    if len < count {
      Err(RenderError::new(format!(
        "{} helper needs at less {} arguments.",
        helper_name, count
      )))
    } else {
      Ok(())
    }
  }

  fn get_param_as_str(&self, index: usize) -> Option<&str> {
    if let Some(Some(s)) = self.param(index).map(|p| p.value().as_str()) {
      Some(s)
    } else {
      None
    }
  }

  fn get_param_as_json(&self, index: usize) -> Option<&Value> {
    self.param(index).map(|p| p.value())
  }
}
