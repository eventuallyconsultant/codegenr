use regex::Regex;

pub trait StringExt {
  fn is_empty_or_whitespaces(&self) -> bool;
  fn split_get_first(&self, splitter: Option<String>) -> String;
  fn split_get_last(&self, splitter: Option<String>) -> String;
  fn get_first_char(&self) -> Option<char>;

  fn trim_char(&self, trimmer: Option<String>) -> String;
  fn trim_start_char(&self, trimmer: Option<String>) -> String;
  fn trim_end_char(&self, trimmer: Option<String>) -> String;

  fn uppercase_first_letter(&self) -> String;
  fn lowercase_first_letter(&self) -> String;
  fn pascal_case(&self) -> String;
  fn camel_case(&self) -> String;
  fn snake_case(&self) -> String;

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>) -> String;

  fn regex_extract(&self, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>) -> Result<String, anyhow::Error>;
  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, anyhow::Error>;
}

// impl<T> StringExt for T where T: AsRef<str> {}

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

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>) -> String {
    self.as_ref().map_or(Default::default(), |s| s.on_one_line(indent, line_break))
  }

  fn regex_extract(&self, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>) -> Result<String, anyhow::Error> {
    self
      .as_ref()
      .map(|s| s.regex_extract(regex_extractor, regex_replacer, separator))
      .transpose()
      .map(|s| s.unwrap_or_default())
  }
  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, anyhow::Error> {
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

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>) -> String {
    self.as_str().on_one_line(indent, line_break)
  }

  fn regex_extract(&self, regex_extractor: &str, regex_replacer: Option<&str>, separator: Option<&str>) -> Result<String, anyhow::Error> {
    self.as_str().regex_extract(regex_extractor, regex_replacer, separator)
  }
  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, anyhow::Error> {
    self.as_str().regex_transform(regex_pattern, regex_replacer)
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

  fn pascal_case(&self) -> String {
    let (_, pascal) = self.trim().chars().fold((Some(true), String::with_capacity(self.len())), |acc, x| {
      let (upper_next, mut s) = acc;
      if is_out_of_case(x) {
        return (Some(true), s);
      }
      match upper_next {
        Some(true) => s.push(x.to_uppercase().next().unwrap_or(x)),
        _ => s.push(x),
      }
      (Some(false), s)
    });
    pascal
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

  fn snake_case(&self) -> String {
    let (_, _, snake) = self
      .trim()
      .chars()
      .fold((Some(true), Some(true), String::with_capacity(self.len())), |acc, x| {
        let (previous_underscore, previous_upper, mut s) = acc;
        if is_out_of_case(x) {
          if !previous_underscore.unwrap_or(true) {
            s.push('_');
          }
          return (Some(true), Some(false), s);
        }

        let is_upper = x.is_uppercase();
        if is_upper && !previous_underscore.unwrap_or(false) && !previous_upper.unwrap_or(false) {
          s.push('_');
        }

        s.push(x.to_lowercase().next().unwrap_or(x));

        (Some(false), Some(is_upper), s)
      });
    snake
  }

  fn on_one_line(&self, indent: Option<u64>, line_break: Option<bool>) -> String {
    let r = ONE_LINER_REGEX.replace_all(self, "");

    let eol = if line_break.unwrap_or(true) { "\n" } else { "" };
    let indent = indent.unwrap_or_default();

    let result = format!("{:indent$}{}{}", "", r.trim(), eol, indent = indent as usize);
    result
  }

  fn regex_extract(&self, regex_extractor: &str, replacer: Option<&str>, separator: Option<&str>) -> Result<String, anyhow::Error> {
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

  fn regex_transform(&self, regex_pattern: &str, regex_replacer: &str) -> Result<String, anyhow::Error> {
    let regex_extr = Regex::new(regex_pattern)?;
    let transformed = regex_extr.replace_all(self, regex_replacer);
    Ok(transformed.into())
  }
}

static ONE_LINER_REGEX: once_cell::sync::Lazy<regex::Regex> =
  once_cell::sync::Lazy::new(|| regex::Regex::new(r#" *[\r\n]+ *"#).expect("The ONE_LINER_REGEX regex did not compile."));

fn is_out_of_case(c: char) -> bool {
  !(c.is_alphabetic() || c.is_numeric())
}

#[cfg(test)]
mod test {
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
  #[test_case("HELLO", "HELLO")]
  #[test_case("HelloWorld", "HelloWorld")]
  #[test_case("helloo", "Helloo")]
  #[test_case("heLlo wOrld", "HeLloWOrld")]
  #[test_case("hello_wwrld", "HelloWwrld")]
  #[test_case("hello-worldd", "HelloWorldd")]
  #[test_case("helo-WORLD", "HeloWORLD")]
  #[test_case("hello/WOOLD", "HelloWOOLD")]
  #[test_case("hello\\\\WORD", "HelloWORD")]
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

  #[test_case("42", "42")]
  #[test_case("hello", "hello")]
  #[test_case("helo world", "helo_world")]
  #[test_case("helloo_world", "helloo_world")]
  #[test_case("heloo-world", "heloo_world")]
  #[test_case("hallo--world", "hallo_world")]
  #[test_case("halo__World", "halo_world")]
  #[test_case("haloo-World", "haloo_world")]
  #[test_case("halloo _ world", "halloo_world")]
  #[test_case("heello - world", "heello_world")]
  #[test_case("HelloWoorld", "hello_woorld")]
  #[test_case("heello _WOORLD", "heello_woorld")]
  #[test_case(" HEELLO", "heello")]
  #[test_case("Helo ", "helo")]
  #[test_case("2Hello2 ", "2_hello2")]
  #[test_case("HelloWorld_42LongName ", "hello_world_42_long_name")]
  fn snake_case_tests(v: &str, expected: &str) {
    assert_eq!(v.snake_case(), expected);
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
