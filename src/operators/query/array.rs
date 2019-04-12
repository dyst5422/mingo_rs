use serde_json::Value;
use external::query;
use utils::from_value;
use error::*;

pub fn all(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  let b_vec: Vec<Value> = from_value(b)?;
  Ok(a_vec.iter().all(|a_val| b_vec.contains(a_val)))
}

pub fn elem_match(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  let b_vec: Vec<Value> = from_value(b)?;
  Ok(b_vec.iter().any(|b_val| query(a, b_val, genealogy)))
}

pub fn size(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let a_usize: usize = from_value(a)?;
  let b_vec: Vec<Value> = from_value(b)?;
  Ok(b_vec.len() == a_usize)
}