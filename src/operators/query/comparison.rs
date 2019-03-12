use internal::query;
use serde_json;
use util;

pub fn eq(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  query(a, b, last_key)
}

pub fn ne(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  Ok(!eq(a, b, last_key)?)
}

pub fn gt(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {

  match a {
    serde_json::Value::Number(a_number) => {
      let a_f64 = util::number_to_f64(a_number)?;
      let b_number = util::value_to_number(b)?;
      let b_f64 = util::number_to_f64(&b_number)?;
      Ok(a_f64 < b_f64)
    },
    serde_json::Value::String(a_string) => {
      let b_string = &util::value_to_string(b)?;
      Ok(a_string < b_string)
    },
    _ => Err("Can only use operator $gt on Number or String".to_string()),
  }
}

pub fn gte(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  match a {
    serde_json::Value::Number(a_number) => {
      let a_f64 = util::number_to_f64(a_number)?;
      let b_number = util::value_to_number(b)?;
      let b_f64 = util::number_to_f64(&b_number)?;
      Ok(a_f64 <= b_f64)
    },
    serde_json::Value::String(a_string) => {
      let b_string = &util::value_to_string(b)?;
      Ok(a_string <= b_string)
    },
    _ => Err("Can only use operator $gte on Number or String".to_string()),
  }
}

pub fn lt(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  match a {
    serde_json::Value::Number(a_number) => {
      let a_f64 = util::number_to_f64(a_number)?;
      let b_number = util::value_to_number(b)?;
      let b_f64 = util::number_to_f64(&b_number)?;
      Ok(a_f64 > b_f64)
    },
    serde_json::Value::String(a_string) => {
      let b_string = &util::value_to_string(b)?;
      Ok(a_string > b_string)
    },
    _ => Err("Can only use operator $gt on Number or String".to_string()),
  }
}

pub fn lte(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  match a {
    serde_json::Value::Number(a_number) => {
      let a_f64 = util::number_to_f64(a_number)?;
      let b_number = util::value_to_number(b)?;
      let b_f64 = util::number_to_f64(&b_number)?;
      Ok(a_f64 >= b_f64)
    },
    serde_json::Value::String(a_string) => {
      let b_string = &util::value_to_string(b)?;
      Ok(a_string >= b_string)
    },
    _ => Err("Can only use operator $lte on Number or String".to_string()),
  }
}

pub fn in_op(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  let a_array = util::value_to_array(a)?;
  Ok(a_array.as_slice().contains(b))
}

pub fn nin_op(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  let a_array = util::value_to_array(a)?;
  Ok(!a_array.as_slice().contains(b))
}
