use serde_json::{Value, from_value};
use internal::query;
use error::*;

pub fn and(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(a_vec.iter().all(|&x| query(x, b, genealogy)))
}

pub fn not(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  Ok(!query(a, b, genealogy))
}

pub fn or(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(a_vec.iter().any(|&x| query(x, b, genealogy)))
}

pub fn nor(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(a_vec.iter().any(|&x| !query(x, b, genealogy)))
}