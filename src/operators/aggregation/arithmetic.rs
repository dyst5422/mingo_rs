use serde_json;
use util;

pub fn abs(a: &serde_json::Value) -> Result<serde_json::Value, String> {
  let a_number = util::value_to_number(a)?;
  let a_f64 = util::number_to_f64(&a_number)?;
  util::f64_to_value(&a_f64.abs())
}

pub fn add(a: &serde_json::Value) -> Result<serde_json::Value, String> {
  let a_array = util::value_to_array(a)?;
  match a_array.iter().try_fold(0.0, |accum, val| {
    let val_number = util::value_to_number(val)?;
    let val_f64 = util::number_to_f64(&val_number)?;
    Ok(accum + val_f64)
  }) {
    Ok(res) => util::f64_to_value(&res),
    Err(msg) => Err(msg),
  }
}

pub fn ceil(a: &serde_json::Value) -> Result<serde_json::Value, String> {
  let a_number = util::value_to_number(a)?;
  let a_f64 = util::number_to_f64(&a_number)?;
  util::f64_to_value(&a_f64.ceil())
}

pub fn divide(a: &serde_json::Value) -> Result<serde_json::Value, String> {
  let a_array = util::value_to_array(a)?;
  match a_array.len() {
    2 => {
      let first_f64 = util::number_to_f64(&util::value_to_number(&a_array[0])?)?;
      let second_f64 = util::number_to_f64(&util::value_to_number(&a_array[1])?)?;
      util::f64_to_value(&(first_f64 / second_f64))
    }
    _ => Err("$divide operator requires exactly two inputs".to_string()),
  }
}
