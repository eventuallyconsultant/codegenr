use handlebars::handlebars_helper;

pub const HEX: &str = "hex";

/// Return the hexadecimal representation of the integer value
/// ```
/// # use codegenr::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(serde_json::Value::Null, "{{hex 42}}"),
///   "0x2a"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": 42 }), "{{hex value}}"),
///   "0x2a"
/// );
/// ```
pub fn hex(v: i64) -> String {
  format!("0x{:x}", v)
}
handlebars_helper!(Hex: |v: i64| hex(v));
