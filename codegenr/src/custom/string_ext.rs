pub trait StringExt {
  fn is_empty_or_whitespaces(&self) -> bool;
}

impl StringExt for String {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.as_str().is_empty_or_whitespaces()
  }
}

impl StringExt for &str {
  fn is_empty_or_whitespaces(&self) -> bool {
    self.is_empty() || self.trim().is_empty()
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
}
