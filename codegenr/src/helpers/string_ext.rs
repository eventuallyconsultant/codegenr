use regex::Regex;

use super::HelpersError;

pub trait StringExt {
  fn is_empty_or_whitespaces(&self) -> bool;

  fn trim_char(&self, trimmer: Option<String>) -> String;
  fn trim_start_char(&self, trimmer: Option<String>) -> String;
  fn trim_end_char(&self, trimmer: Option<String>) -> String;

  fn uppercase_first_letter(&self) -> String;
  fn lowercase_first_letter(&self) -> String;

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>, replacer: Option<&str>) -> String;

  fn regex_extract(&self, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>) -> Result<String, HelpersError>;
  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, HelpersError>;
}

// impl<T> StringExt for T where T: AsRef<str> {}

impl StringExt for Option<String> {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.as_ref().map_or(true, |s| s.is_empty_or_whitespaces())
  }

  fn trim_char(&self, trimmer: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.trim_char(trimmer))
  }

  fn trim_start_char(&self, trimmer: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.trim_start_char(trimmer))
  }

  fn trim_end_char(&self, trimmer: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.trim_end_char(trimmer))
  }

  fn uppercase_first_letter(&self) -> String {
    self.as_ref().map_or(Default::default(), |s| s.uppercase_first_letter())
  }

  fn lowercase_first_letter(&self) -> String {
    self.as_ref().map_or(Default::default(), |s| s.lowercase_first_letter())
  }

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>, replacer: Option<&str>) -> String {
    self
      .as_ref()
      .map_or(Default::default(), |s| s.on_one_line(indent, line_break, replacer))
  }

  fn regex_extract(&self, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>) -> Result<String, HelpersError> {
    self
      .as_ref()
      .map(|s| s.regex_extract(regex_extractor, regex_replacer, separator))
      .transpose()
      .map(|s| s.unwrap_or_default())
  }
  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, HelpersError> {
    self
      .as_ref()
      .map(|s| s.regex_transform(regex_pattern, regex_replacer))
      .transpose()
      .map(|s| s.unwrap_or_default())
  }
}

impl StringExt for String {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.as_str().is_empty_or_whitespaces()
  }

  fn trim_char(&self, trimmer: Option<String>) -> String {
    self.as_str().trim_char(trimmer)
  }

  fn trim_start_char(&self, trimmer: Option<String>) -> String {
    self.as_str().trim_start_char(trimmer)
  }

  fn trim_end_char(&self, trimmer: Option<String>) -> String {
    self.as_str().trim_end_char(trimmer)
  }

  fn uppercase_first_letter(&self) -> String {
    self.as_str().uppercase_first_letter()
  }

  fn lowercase_first_letter(&self) -> String {
    self.as_str().lowercase_first_letter()
  }

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>, replacer: Option<&str>) -> String {
    self.as_str().on_one_line(indent, line_break, replacer)
  }

  fn regex_extract(&self, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>) -> Result<String, HelpersError> {
    self.as_str().regex_extract(regex_extractor, regex_replacer, separator)
  }
  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, HelpersError> {
    self.as_str().regex_transform(regex_pattern, regex_replacer)
  }
}

impl StringExt for &str {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.is_empty() || self.trim().is_empty()
  }

  fn trim_char(&self, trimmer: Option<String>) -> String {
    let trimmer = trimmer.unwrap_or_else(|| " ".to_string()).chars().next().unwrap_or(' ');
    self.trim_matches(trimmer).to_string()
  }

  fn trim_start_char(&self, trimmer: Option<String>) -> String {
    let trimmer = trimmer.unwrap_or_else(|| " ".to_string()).chars().next().unwrap_or(' ');
    self.trim_start_matches(trimmer).to_string()
  }

  fn trim_end_char(&self, trimmer: Option<String>) -> String {
    let trimmer = trimmer.unwrap_or_else(|| " ".to_string()).chars().next().unwrap_or(' ');
    self.trim_end_matches(trimmer).to_string()
  }

  fn uppercase_first_letter(&self) -> String {
    if self.is_empty_or_whitespaces() {
      return String::default();
    }
    let mut ve: Vec<char> = self.chars().collect();
    ve[0] = ve[0].to_uppercase().next().unwrap();
    let result: String = ve.into_iter().collect();
    result
  }

  fn lowercase_first_letter(&self) -> String {
    if self.is_empty_or_whitespaces() {
      return String::default();
    }
    let mut ve: Vec<char> = self.chars().collect();
    ve[0] = ve[0].to_lowercase().next().unwrap();
    let result: String = ve.into_iter().collect();
    result
  }

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>, replacer: Option<&str>) -> String {
    let replacer = replacer.unwrap_or("");
    let r = ONE_LINER_REGEX.replace_all(self, replacer);

    let eol = if line_break.unwrap_or(true) { "\n" } else { "" };
    let indent = indent.unwrap_or_default();

    let result = format!("{:indent$}{}{}", "", r.trim(), eol, indent = indent as usize);
    result
  }

  fn regex_extract(&self, regex_extractor: &str, replacer: Option<&str>, separator: Option<&str>) -> Result<String, HelpersError> {
    let regex_extr = Regex::new(regex_extractor)?;
    let replacer = replacer.unwrap_or("$1");
    let separator = separator.unwrap_or(", ");

    let mut values = Vec::new();
    for m in regex_extr.find_iter(self) {
      let matched = m.as_str();
      let replaced = regex_extr.replace(matched, replacer);
      values.push(replaced.to_string());
    }

    Ok(values.join(separator))
  }

  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, HelpersError> {
    let regex_extr = Regex::new(regex_pattern)?;
    let transformed = regex_extr.replace_all(self, regex_replacer);
    Ok(transformed.into())
  }
}

static ONE_LINER_REGEX: once_cell::sync::Lazy<regex::Regex> =
  once_cell::sync::Lazy::new(|| regex::Regex::new(r#" *[\r\n]+ *"#).expect("The ONE_LINER_REGEX regex did not compile."));

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("/user/{username}", "\\{([^}]*)}", Some("$1"), None, "username")]
  #[test_case("/user/{username}/{id}", "\\{([^}]*)}", Some("$1"), None, "username, id")]
  #[test_case("/user/{username}/{id}", "\\{([^}]*)}", Some("<$1>"), Some("|"), "<username>|<id>")]
  fn regex_extract_tests(arg: &str, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>, expected: &str) {
    let result = arg.regex_extract(regex_extractor, regex_replacer, separator).unwrap();
    assert_eq!(result, expected);
  }

  #[test_case("/user/{username}", "\\{([^}]*)}", "$1", "/user/username")]
  #[test_case("/user/{user}/{id}", "\\{([^}]*)}", "$1", "/user/user/id")]
  #[test_case("/user/{username}/{id}", "\\{([^}]*)}", "<$1>", "/user/<username>/<id>")]
  fn regex_transform_tests(arg: &str, regex_extractor: &str, regex_replacer: &str, expected: &str) {
    let result = arg.regex_transform(regex_extractor, regex_replacer).unwrap();
    assert_eq!(result, expected);
  }

  #[test_case(" ", true)]
  #[test_case("  \t\n ", true)]
  #[test_case("  \n", true)]
  #[test_case("hello", false)]
  fn is_empty_or_whitespaces_tests(v: &str, expected: bool) {
    assert_eq!(v.is_empty_or_whitespaces(), expected);
    assert_eq!(v.to_string().is_empty_or_whitespaces(), expected);
    assert_eq!(Some(v.to_string()).is_empty_or_whitespaces(), expected);
  }

  #[test_case("", "e", "")]
  #[test_case(" leave ", "", "leave")]
  #[test_case("elle", "e", "ll")]
  #[test_case("-test_", "-", "test_")]
  #[test_case("leel", "e", "leel")]
  //todo: #[test_case("test", " t", "es")]
  fn del_char_tests(v: &str, trimmer: &str, expected: &str) {
    let trimmer = if trimmer.is_empty_or_whitespaces() {
      None
    } else {
      Some(trimmer.to_string())
    };
    assert_eq!(v.trim_char(trimmer), expected);
  }

  #[test_case("", "e", "")]
  #[test_case(" leave ", "", "leave ")]
  #[test_case("elle", "e", "lle")]
  #[test_case("-test_", "_", "-test_")]
  #[test_case("leel", "e", "leel")]
  fn del_char_start_tests(v: &str, trimmer: &str, expected: &str) {
    let trimmer = if trimmer.is_empty_or_whitespaces() {
      None
    } else {
      Some(trimmer.to_string())
    };
    assert_eq!(v.trim_start_char(trimmer), expected);
  }

  #[test_case("", "e", "")]
  #[test_case(" leave ", "", " leave")]
  #[test_case("elle", "e", "ell")]
  #[test_case("-test_", "-", "-test_")]
  #[test_case("leel", "e", "leel")]
  fn del_char_end_tests(v: &str, trimmer: &str, expected: &str) {
    let trimmer = if trimmer.is_empty_or_whitespaces() {
      None
    } else {
      Some(trimmer.to_string())
    };
    assert_eq!(v.trim_end_char(trimmer), expected);
  }

  #[test_case("42", "42")]
  #[test_case("hello", "hello")]
  #[test_case("Test", "test")]
  fn lowercase_first_letter_tests(v: &str, expected: &str) {
    assert_eq!(v.lowercase_first_letter(), expected)
  }

  #[test_case("42", "42")]
  #[test_case("HELLO", "HELLO")]
  #[test_case("test", "Test")]
  fn uppercase_first_letter_tests(v: &str, expected: &str) {
    assert_eq!(v.uppercase_first_letter(), expected)
  }
}
