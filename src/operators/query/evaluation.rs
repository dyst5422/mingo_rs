use serde_json::{Value, from_value};
use regex;
use internal::{aggregate, query};
use error::*;

pub fn expr(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_aggregate = aggregate(a, b, genealogy)?;
  Ok(query(a_aggregate, b, genealogy))
}

pub fn json_schema(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn mod_op(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn regex_op(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_str: String = from_value(a)?;
  let b_str: String = from_value(b)?;

  Ok(regex::Regex::new(&a_str)?.is_match(&b_str))
}

pub fn text(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn where_op(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  unimplemented!()
}
