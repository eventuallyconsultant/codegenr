pub trait StringExt {
  fn is_empty_or_whitespaces(&self) -> bool;
  fn split_get_first(&self, splitter: Option<String>) -> String;
  fn split_get_last(&self, splitter: Option<String>) -> String;
  fn get_first_char(&self) -> Option<char>;
}

impl StringExt for Option<String> {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.as_ref().map_or(true, |s| s.is_empty_or_whitespaces())
  }

  fn split_get_first(&self, splitter: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.split_get_first(splitter))
  }

  fn split_get_last(&self, splitter: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.split_get_last(splitter))
  }

  fn get_first_char(&self) -> Option<char> {
    self.as_ref().and_then(|s| s.get_first_char())
  }
}

impl StringExt for String {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.as_str().is_empty_or_whitespaces()
  }

  fn split_get_first(&self, splitter: Option<String>) -> String {
    self.as_str().split_get_first(splitter)
  }

  fn split_get_last(&self, splitter: Option<String>) -> String {
    self.as_str().split_get_last(splitter)
  }

  fn get_first_char(&self) -> Option<char> {
    self.as_str().get_first_char()
  }
}

impl StringExt for &str {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.is_empty() || self.trim().is_empty()
  }

  fn split_get_first(&self, splitter: Option<String>) -> String {
    let c = splitter.get_first_char().unwrap_or('/');
    self.split(c).find(|s| !s.is_empty_or_whitespaces()).unwrap_or_default().to_string()
  }

  fn split_get_last(&self, splitter: Option<String>) -> String {
    let c = splitter.get_first_char().unwrap_or('/');
    self
      .split(c)
      .filter(|s| !s.is_empty_or_whitespaces())
      .last()
      .unwrap_or_default()
      .to_string()
  }

  fn get_first_char(&self) -> Option<char> {
    self.chars().next()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use test_case::test_case;

  #[test_case(" ", true)]
  #[test_case("  \t\n ", true)]
  #[test_case("  \n", true)]
  #[test_case("hello", false)]
  fn is_empty_or_whitespaces_tests(v: &str, expected: bool) {
    assert_eq!(v.is_empty_or_whitespaces(), expected);
  }

  #[test_case("leave/me/alone", "", "leave")]
  #[test_case("/leave/me", "", "leave")]
  #[test_case("/leave/me", "e", "/l")]
  #[test_case("", "e", "")]
  fn split_get_first_tests(v: &str, splitter: &str, expected: &str) {
    let splitter = if splitter.is_empty() { None } else { Some(splitter.to_string()) };
    assert_eq!(v.split_get_first(splitter), expected);
  }

  #[test_case("leave/me/alone", "", "alone")]
  #[test_case("/leave/me/", "", "me")]
  #[test_case("/leave/me", "e", "/m")]
  #[test_case("", "e", "")]
  fn split_get_last_tests(v: &str, splitter: &str, expected: &str) {
    let splitter = if splitter.is_empty() { None } else { Some(splitter.to_string()) };
    assert_eq!(v.split_get_last(splitter), expected);
  }
}
