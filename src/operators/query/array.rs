use serde_json;
use internal::query;
use util;

pub fn all(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  let a_array = util::value_to_array(a)?;
  let b_array = util::value_to_array(b)?;
  Ok(b_array.into_iter().all(|b_val| a_array.as_slice().contains(&b_val)))
}

pub fn elem_match(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  let b_array = util::value_to_array(b)?;

  Ok(b_array.into_iter().any(|b_val| match query(a, &b_val, last_key) {
    Ok(val) => val,
    Err(_msg) => false,
  }))
}

pub fn size(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  let a_number = util::value_to_number(a)?;
  let a_u64 = util::number_to_u64(&a_number)?;
  let b_array = util::value_to_array(b)?;
  Ok(b_array.len() == a_u64 as usize)
}