use serde_json;
use regex;
use internal;
use util;

pub fn expr(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  let a_aggregate = &internal::aggregate(a, b)?;
  internal::query(a_aggregate, b, last_key)
}

pub fn json_schema(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  unimplemented!()
}

pub fn mod_op(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  unimplemented!()
}

pub fn regex_op(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  let a_string = util::value_to_string(a)?;
  let b_string = util::value_to_string(b)?;

  match regex::Regex::new(&a_string) {
    Ok(reg_expr) => Ok(reg_expr.is_match(&b_string)),
    Err(err) => match err {
      regex::Error::Syntax(msg) => Err(msg),
      regex::Error::CompiledTooBig(size) => Err(format!("Regex too big: {}", size)),
      _ => panic!("Unknown Regex Error",)
    }
  }
}

pub fn text(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  unimplemented!()
}

pub fn where_op(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  unimplemented!()
}
