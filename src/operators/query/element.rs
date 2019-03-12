use serde_json;

pub fn exists(a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  match last_key {
    Some(key) => match a {
      serde_json::Value::Bool(a_bool) => match b {
        serde_json::Value::Object(b_obj) => match a_bool {
          true => Ok(b_obj.contains_key(key)),
          false => Ok(!b_obj.contains_key(key)),
        },
        _ => Err("Can only check for field existance on Object".to_string()),
      },
      _ => Err("Operation $exists requires Bool parameter. { 'key': { '$exists': true } }".to_string()),
    },
    None => Err("Operation $exists requires a preceding key. { 'key': { '$exists': true } }".to_string()),
  } 
}

pub fn type_op(a: &serde_json::Value, b: &serde_json::Value, _last_key: Option<&str>) -> Result<bool, String> {
  match a {
    serde_json::Value::String(a_str) => match a_str.as_str() {
      "double" => match b {
        serde_json::Value::Number(_b_num) => Ok(true),
        _ => Ok(false),
      },
      "string" => match b {
        serde_json::Value::String(_b_str) => Ok(true),
        _ => Ok(false),
      },
      "object" => match b {
        serde_json::Value::Object(_b_obj) => Ok(true),
        _ => Ok(false),
      },
      "array" => match b {
        serde_json::Value::Array(_b_arr) => Ok(true),
        _ => Ok(false),
      },
      "binData" => unimplemented!(),
      "undefined" => unimplemented!("'undefined' type deprecated"),
      "objectId" => unimplemented!(),
      "bool" => match b {
        serde_json::Value::Bool(_b_bool) => Ok(true),
        _ => Ok(false),
      },
      "date" => unimplemented!(),
      "null" => match b {
        serde_json::Value::Null => Ok(true),
        _ => Ok(false),
      },
      "regex" => unimplemented!(),
      "dbPointer" => unimplemented!("'dbPointer' type deprecated"),
      "javascript" => unimplemented!(),
      "symbol" => unimplemented!("'symbol' type deprecated"),
      "javascriptWithScope" => unimplemented!(),
      "int" => unimplemented!(),
      "timestamp" => unimplemented!(),
      "long" => match b {
        serde_json::Value::Number(b_num) => Ok(b_num.is_i64()),
        _ => Ok(false),
      },
      "decimal" => unimplemented!(),
      "minKey" => unimplemented!(),
      "maxKey" => unimplemented!(),
      "number" => match b {
        serde_json::Value::Number(_b_num) => Ok(true),
        _ => Ok(false),
      },
      _ => Err("Unknown type string".to_string()),
    },
    serde_json::Value::Number(a_num) => match a_num.as_i64() {
      Some(a_int) => match a_int {
        1 => match b {
          serde_json::Value::Number(_b_num) => Ok(true),
          _ => Ok(false),
        },
        2 => match b {
          serde_json::Value::String(_b_str) => Ok(true),
          _ => Ok(false),
        },
        3 => match b {
          serde_json::Value::Object(_b_obj) => Ok(true),
          _ => Ok(false),
        },
        4 => match b {
          serde_json::Value::Array(_b_arr) => Ok(true),
          _ => Ok(false),
        },
        5 => unimplemented!(),
        6 => unimplemented!("'undefined' type deprecated"),
        7 => unimplemented!(),
        8 => match b {
          serde_json::Value::Bool(_b_bool) => Ok(true),
          _ => Ok(false),
        },
        9 => unimplemented!(),
        10 => match b {
          serde_json::Value::Null => Ok(true),
          _ => Ok(false),
        },
        11 => unimplemented!(),
        12 => unimplemented!("'dbPointer' type deprecated"),
        13 => unimplemented!(),
        14 => unimplemented!("'symbol' type deprecated"),
        15 => unimplemented!(),
        16 => unimplemented!(),
        17 => unimplemented!(),
        18 => match b {
          serde_json::Value::Number(b_num) => Ok(b_num.is_i64()),
          _ => Ok(false),
        },
        19 => unimplemented!(),
        -1 => unimplemented!(),
        127 => unimplemented!(),
        _ => Err("Unknown type integer".to_string()),
      },
      None => Err("Must use an Integer type".to_string()),
    },
    _ => Err("Operation $type requires String or Integer parameter. { 'key': { '$type': 'string' } } or { 'key': { '$type': 2 } }".to_string()),
  }
}