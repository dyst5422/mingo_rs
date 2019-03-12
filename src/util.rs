use serde_json;

pub fn number_to_f64(a_number: &serde_json::Number) -> Result<f64, String> {
  match a_number.as_f64() {
    Some(a_f64) => Ok(a_f64),
    None => Err("Cannot convert number to f64".to_string()),
  }
}

pub fn number_to_u64(a_number: &serde_json::Number) -> Result<u64, String> {
  match a_number.as_u64() {
    Some(a_u64) => Ok(a_u64),
    None => Err("Cannot convert number to f64".to_string()),
  }
}

pub fn f64_to_value(a_f64: &f64) -> Result<serde_json::Value, String> {
  match serde_json::to_value(a_f64) {
    Ok(a_value) => Ok(a_value),
    Err(_err) => Err("Cannot convert f64 to serde Value".to_string()),
  }
}

pub fn value_to_number(a_value: &serde_json::Value) -> Result<serde_json::Number, String> {
  match a_value {
    serde_json::Value::Number(a_number) => Ok(a_number.clone()),
    _ => Err("Cannot convert Value to Number".to_string()),
  }
}

pub fn value_to_string(a_value: &serde_json::Value) -> Result<String, String> {
  match a_value {
    serde_json::Value::String(a_string) => Ok(a_string.clone()),
    _ => Err("Cannot convert Value to Number".to_string()),
  }
}

pub fn value_to_array(a_value: &serde_json::Value) -> Result<Vec<serde_json::Value>, String> {
  match a_value {
    serde_json::Value::Array(a_arr) => Ok(a_arr.clone()),
    _ => Err("Cannot convert Value to Array".to_string()),
  }
}

pub fn value_to_object(a_value: &serde_json::Value) -> Result<serde_json::Map<String, serde_json::Value>, String> {
  match a_value {
    serde_json::Value::Object(a_arr) => Ok(a_arr.clone()),
    _ => Err("Cannot convert Value to Object".to_string()),
  }
}
