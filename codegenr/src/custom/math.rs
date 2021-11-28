use handlebars::handlebars_helper;

/// Return the hexadecimal representation of the integer value
/// ```
/// # use codegenr::custom::*;
/// # use serde_json::json;
/// assert_eq!(
///   test_helper(serde_json::Value::Null, "{{hex 42}}"),
///   "0x2a"
/// );
/// assert_eq!(
///   test_helper(json!({ "value": 42 }), "{{hex value}}"),
///   "0x2a"
/// );
/// ```
pub fn hex(v: i64) -> String {
  format!("0x{:x}", v)
}
handlebars_helper!(Hex: |v: i64| hex(v));
