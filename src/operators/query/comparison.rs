use error::*;
use external::query;
use serde_json::Value;
use utils::from_value;

pub fn eq(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  Ok(query(a, b, genealogy))
}

pub fn ne(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  Ok(!eq(a, b, genealogy)?)
}

pub fn gt(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  match a {
    Value::Number(_) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 < b_f64)
    }
    Value::String(a_string) => {
      let b_string: String = from_value(b)?;
      Ok(a_string < &b_string)
    }
    _ => Err(MingoError::ArgumentError {
      genealogy: genealogy.join(","),
      message: "$gt: only usable on numbers or strings",
      argument: a.to_owned(),
    }.into()),
  }
}

pub fn gte(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  match a {
    Value::Number(_) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 <= b_f64)
    }
    Value::String(a_string) => {
      let b_string: String = from_value(b)?;
      Ok(a_string <= &b_string)
    }
    _ => Err(MingoError::ArgumentError {
      genealogy: genealogy.join(","),
      message: "$gte: only usable on numbers or strings",
      argument: a.to_owned(),
    }.into()),
  }
}

pub fn lt(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  match a {
    Value::Number(_) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 > b_f64)
    }
    Value::String(a_string) => {
      let b_string: String = from_value(b)?;
      Ok(a_string > &b_string)
    }
    _ => Err(MingoError::ArgumentError {
      genealogy: genealogy.join(","),
      message: "$lt: only usable on numbers or strings",
      argument: a.to_owned(),
    }.into()),
  }
}

pub fn lte(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  match a {
    Value::Number(_) => {
      let a_f64: f64 = from_value(a)?;
      let b_f64: f64 = from_value(b)?;
      Ok(a_f64 >= b_f64)
    }
    Value::String(a_string) => {
      let b_string: String = from_value(b)?;
      Ok(a_string >= &b_string)
    }
    _ => Err(MingoError::ArgumentError {
      genealogy: genealogy.join(","),
      message: "$lte: only usable on numbers or strings",
      argument: a.to_owned(),
    }.into()),
  }
}

pub fn in_op(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(a_vec.contains(&b))
}

pub fn nin_op(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let a_vec: Vec<Value> = from_value(a)?;
  Ok(!a_vec.contains(&b))
}
