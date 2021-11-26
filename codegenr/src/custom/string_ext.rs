pub trait StringExt {
  fn is_empty_or_whitespaces(&self) -> bool;
  fn split_get_first(&self, splitter: Option<String>) -> String;
  fn split_get_last(&self, splitter: Option<String>) -> String;
  fn get_first_char(&self) -> Option<char>;

  // TODO :
  // trim() functions (not named 'trim' to avoid messing with already existing trim() func)
  fn trim_char(&self, trimmer: Option<String>) -> String;
  fn trim_start_char(&self, trimmer: Option<String>) -> String;
  fn trim_end_char(&self, trimmer: Option<String>) -> String;

  fn uppercase_first_letter(&self) -> String;
  fn lowercase_first_letter(&self) -> String;
  fn pascal_case(&self) -> String;
  fn camel_case(&self) -> String;
  fn snake_case(&self) -> String;
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

  fn pascal_case(&self) -> String {
    self.as_ref().map_or(Default::default(), |s| s.pascal_case())
  }

  fn camel_case(&self) -> String {
    self.as_ref().map_or(Default::default(), |s| s.camel_case())
  }

  fn snake_case(&self) -> String {
    self.as_ref().map_or(Default::default(), |s| s.snake_case())
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

  fn pascal_case(&self) -> String {
    self.as_str().pascal_case()
  }

  fn camel_case(&self) -> String {
    self.as_str().camel_case()
  }

  fn snake_case(&self) -> String {
    self.as_str().snake_case()
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

  fn trim_char(&self, trimmer: Option<String>) -> String {
    todo!()
  }

  fn trim_start_char(&self, trimmer: Option<String>) -> String {
    todo!()
  }

  fn trim_end_char(&self, trimmer: Option<String>) -> String {
    todo!()
  }

  fn uppercase_first_letter(&self) -> String {
    todo!()
  }

  fn lowercase_first_letter(&self) -> String {
    todo!()
  }

  fn pascal_case(&self) -> String {
    todo!()
  }

  fn camel_case(&self) -> String {
    todo!()
  }

  fn snake_case(&self) -> String {
    todo!()
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
    assert_eq!(v.to_string().is_empty_or_whitespaces(), expected);
    assert_eq!(Some(v.to_string()).is_empty_or_whitespaces(), expected);
  }

  #[test_case("leave/me/alone", "", "leave")]
  #[test_case("/leave/me", "", "leave")]
  #[test_case("/leave/me", "e", "/l")]
  #[test_case("", "e", "")]
  fn split_get_first_tests(v: &str, splitter: &str, expected: &str) {
    let splitter = if splitter.is_empty() { None } else { Some(splitter.to_string()) };
    assert_eq!(v.split_get_first(splitter.clone()), expected);
    assert_eq!(v.to_string().split_get_first(splitter.clone()), expected);
    assert_eq!(Some(v.to_string()).split_get_first(splitter), expected);
  }

  #[test_case("leave/me/alone", "", "alone")]
  #[test_case("/leave/me/", "", "me")]
  #[test_case("/leave/me", "e", "/m")]
  #[test_case("", "e", "")]
  fn split_get_last_tests(v: &str, splitter: &str, expected: &str) {
    let splitter = if splitter.is_empty() { None } else { Some(splitter.to_string()) };
    assert_eq!(v.split_get_last(splitter.clone()), expected);
    assert_eq!(v.to_string().split_get_last(splitter.clone()), expected);
    assert_eq!(Some(v.to_string()).split_get_last(splitter), expected);
  }
}
