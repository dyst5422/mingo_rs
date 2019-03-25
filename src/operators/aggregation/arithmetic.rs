use serde_json::{Value, from_value};
use error::*;

pub fn abs(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.abs().into())
}

pub fn add(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_vec: Vec<f64> = from_value(a)?;
  Ok(a_vec.iter().fold(0.0, |accum, val| accum + val).into())
}

pub fn ceil(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.ceil().into())
}

pub fn divide(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_f64, b_f64): (f64, f64) = from_value(a)?;
  Ok((a_f64 / b_f64).into())
}

pub fn exp(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.exp().into())
}

pub fn floor(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.floor().into())
}

pub fn ln(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.ln().into())
}

pub fn log(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_f64, b_f64): (f64, f64) = from_value(a)?;
  Ok(a_f64.log(b_f64).into())
}

pub fn log10(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.log10().into())
}

pub fn mod_op(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_f64, b_f64): (f64, f64) = from_value(a)?;
  Ok((a_f64 % b_f64).into())
}

pub fn multiply(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_f64, b_f64): (f64, f64) = from_value(a)?;
  Ok((a_f64 * b_f64).into())
}

pub fn pow(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_f64, b_f64): (f64, f64) = from_value(a)?;
  Ok(a_f64.powf(b_f64).into())
}

pub fn sqrt(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.sqrt().into())
}

pub fn subtract(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_f64, b_f64): (f64, f64) = from_value(a)?;
  Ok((a_f64 - b_f64).into())
}

pub fn trunc(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_f64: f64 = from_value(a)?;
  Ok(a_f64.trunc().into())
}
