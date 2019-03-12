use serde_json;
use internal;
use util;

pub fn and(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  let a_array = util::value_to_array(a)?;
  Ok(a_array.into_iter().all(|x| match internal::query(&x, b, last_key) {
    Ok(val) => val,
    Err(_msg) => false,
  }))
}

pub fn not(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  Ok(!internal::query(a, b, last_key)?)
}

pub fn or(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  let a_array = util::value_to_array(a)?;
  Ok(a_array.into_iter().any(|x| match internal::query(&x, b, last_key) {
    Ok(val) => val,
    Err(_msg) => false,
  }))
}

pub fn nor(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  let a_array = util::value_to_array(a)?;
  Ok(!a_array.into_iter().any(|x| match internal::query(&x, b, last_key) {
    Ok(val) => val,
    Err(_msg) => false,
  }))
}