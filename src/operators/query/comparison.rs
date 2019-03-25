use internal::query;
use serde_json::{Value, from_value};
use error::*;

pub fn eq(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  Ok(query(a, b, genealogy))
}

pub fn ne(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  Ok(!eq(a, b, genealogy)?)
}

pub fn gt(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  match a {
    Value::Number(a_number) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 < b_f64)
    },
    Value::String(a_string) => {
      let a_string: String = from_value(a)?;
      let b_string: String = from_value(b)?;
      Ok(a_string < b_string)
    },
  }
}

pub fn gte(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  match a {
    Value::Number(a_number) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 <= b_f64)
    },
    Value::String(a_string) => {
      let a_string: String = from_value(a)?;
      let b_string: String = from_value(b)?;
      Ok(a_string <= b_string)
    },
  }
}

pub fn lt(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  match a {
    Value::Number(a_number) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 > b_f64)
    },
    Value::String(a_string) => {
      let a_string: String = from_value(a)?;
      let b_string: String = from_value(b)?;
      Ok(a_string > b_string)
    },
  }
}

pub fn lte(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  match a {
    Value::Number(a_number) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 >= b_f64)
    },
    Value::String(a_string) => {
      let a_string: String = from_value(a)?;
      let b_string: String = from_value(b)?;
      Ok(a_string >= b_string)
    },
  }
}

pub fn in_op(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(a_vec.contains(&b))
}

pub fn nin_op(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(!a_vec.contains(&b))
}
