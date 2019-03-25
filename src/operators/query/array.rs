use serde_json::{Value, from_value};
use internal::query;
use error::*;

pub fn all(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  let b_vec: Vec<Value> = from_value(b)?;
  Ok(b_vec.iter().all(|b_val| a_vec.contains(b_val)))
}

pub fn elem_match(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let b_vec: Vec<Value> = from_value(b)?;
  Ok(b_vec.iter().any(|&b_val| query(a, b_val, genealogy)))
}

pub fn size(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_usize: usize = from_value(a)?;
  let b_vec: Vec<Value> = from_value(b)?;
  Ok(b_vec.len() == a_usize)
}