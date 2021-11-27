pub trait StringExt {
  fn is_empty_or_whitespaces(&self) -> bool;
  fn split_get_first(&self, splitter: Option<String>) -> String;
  fn split_get_last(&self, splitter: Option<String>) -> String;
  fn get_first_char(&self) -> Option<char>;

  fn del_char(&self, trimmer: Option<String>) -> String;
  fn del_start_char(&self, trimmer: Option<String>) -> String;
  fn del_end_char(&self, trimmer: Option<String>) -> String;

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

  fn del_char(&self, trimmer: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.del_char(trimmer))
  }

  fn del_start_char(&self, trimmer: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.del_start_char(trimmer))
  }

  fn del_end_char(&self, trimmer: Option<String>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.del_end_char(trimmer))
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

  fn del_char(&self, trimmer: Option<String>) -> String {
    self.as_str().del_char(trimmer)
  }

  fn del_start_char(&self, trimmer: Option<String>) -> String {
    self.as_str().del_start_char(trimmer)
  }

  fn del_end_char(&self, trimmer: Option<String>) -> String {
    self.as_str().del_end_char(trimmer)
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

  fn del_char(&self, trimmer: Option<String>) -> String {
    let trimmer = trimmer.unwrap_or_else(|| " ".to_string()).chars().next().unwrap_or(' ');
    self.trim_matches(trimmer).to_string()
  }

  fn del_start_char(&self, trimmer: Option<String>) -> String {
    let trimmer = trimmer.unwrap_or_else(|| " ".to_string()).chars().next().unwrap_or(' ');
    self.trim_start_matches(trimmer).to_string()
  }

  fn del_end_char(&self, trimmer: Option<String>) -> String {
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

  fn pascal_case(&self) -> String {
    if self.is_empty_or_whitespaces() {
      return String::default();
    }
    let mut ve: Vec<char> = self.to_lowercase().chars().collect();
    ve[0] = ve[0].to_uppercase().next().unwrap();
    let result: String = ve.into_iter().collect();
    result
  }

  fn camel_case(&self) -> String {
    let (_, camel) = self
      .trim()
      .chars()
      .fold((Some(false), String::with_capacity(self.len())), |acc, x| {
        let (upper_next, mut s) = acc;
        if is_out_of_case(x) {
          return (Some(true), s);
        }

        match upper_next {
          Some(up) => {
            let c = if up {
              x.to_uppercase().next().unwrap_or(x)
            } else {
              x.to_lowercase().next().unwrap_or(x)
            };
            s.push(c);
          }
          None => s.push(x),
        }
        (None, s)
      });
    camel
  }

  // [HandlebarsHelperSpecification("{ test: 42 }", "{{snake_case test}}", "42")]
  //   [HandlebarsHelperSpecification("{ test: 'hello ' }", "{{snake_case test}}", "hello")]
  //   [HandlebarsHelperSpecification("{ test: 'hello world' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello_world' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello-world' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello--world' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello__World' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello-World' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello _ world' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello - world' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'HelloWorld' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: 'hello _WORLD' }", "{{snake_case test}}", "hello_world")]
  //   [HandlebarsHelperSpecification("{ test: ' HELLO' }", "{{snake_case test}}", "hello")]
  //   [HandlebarsHelperSpecification("{ test: 'Hello ' }", "{{snake_case test}}", "hello")]
  //   [HandlebarsHelperSpecification("{ test: '2Hello2 ' }", "{{snake_case test}}", "2_hello2")]
  //   [HandlebarsHelperSpecification("{ test: 'HelloWorld_42LongName ' }", "{{snake_case test}}", "hello_world_42_long_name")]
  fn snake_case(&self) -> String {
    todo!()
  }
}

fn is_out_of_case(c: char) -> bool {
  c == ' ' || c == '_' || c == '-' || c == '\\' || c == '|' || c == '/'
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

  #[test_case("", "e", "")]
  #[test_case(" leave ", "", "leave")]
  #[test_case("elle", "e", "ll")]
  #[test_case("-test_", "-", "test_")]
  #[test_case("leel", "e", "leel")]
  #[test_case("test", " t", "es")]
  fn del_char_tests(v: &str, trimmer: &str, expected: &str) {
    let trimmer = if trimmer.is_empty_or_whitespaces() {
      None
    } else {
      Some(trimmer.to_string())
    };
    assert_eq!(v.del_char(trimmer), expected);
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
    assert_eq!(v.del_start_char(trimmer), expected);
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
    assert_eq!(v.del_end_char(trimmer), expected);
  }

  #[test_case("leave", "Leave")]
  #[test_case("eLlE", "Elle")]
  #[test_case("/test", "Test")]
  #[test_case("42lol", "42lol")]
  // more tests
  #[test_case("42", "42")]
  #[test_case("HELLO", "Hello")]
  #[test_case("HelloWorld", "HelloWorld")]
  #[test_case("helloo", "Helloo")]
  #[test_case("heLlo wOrld", "HeLloWOrld")]
  #[test_case("hello_wwrld", "HelloWwrld")]
  #[test_case("hello-worldd", "HelloWorldd")]
  #[test_case("helo-WORLD", "Helo-WORLD")]
  #[test_case("hello/WOOLD", "helloWOOLD")]
  #[test_case("hello\\\\WORD", "HelloWORLD")]
  fn pascal_case_tests(v: &str, expected: &str) {
    assert_eq!(v.pascal_case(), expected);
  }

  #[test_case("42", "42")]
  #[test_case("HELLO", "hELLO")]
  #[test_case("hey", "hey")]
  #[test_case("heLlo wOrld", "heLloWOrld")]
  #[test_case("hey_world", "heyWorld")]
  #[test_case("helo-world", "heloWorld")]
  #[test_case("helloo-WORLD", "hellooWORLD")]
  #[test_case("HelooWorld", "helooWorld")]
  fn camel_case_tests(v: &str, expected: &str) {
    assert_eq!(v.camel_case(), expected);
  }

  // todo: rewrite tests
  // #[test_case("leave", "Leave")]
  // #[test_case("eLlE", "Elle")]
  // #[test_case("/test", "/test")]
  // #[test_case("42lol", "42lol")]
  fn snake_case_tests(v: &str, expected: &str) {
    assert_eq!(v.snake_case(), expected);
  }
}
