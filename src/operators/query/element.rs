use serde_json::{Value, from_value, Map};
use error::*;

pub fn exists(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {

    let a_string: String = from_value(a)?;
    let b_obj: Map<String, Value> = from_value(b)?;
    Ok(b_obj.contains_key(match genealogy.last() {
      Some(val) => val,
    }))
}

pub fn type_op(a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  match a {
    Value::String(a_str) => {
      let a_string: String = from_value(a)?;
      Ok(match a_string.as_str() {
        "double" => b.is_f64(),
        "string" => b.is_string(),
        "object" => b.is_object(),
        "array" => b.is_array(),
        "binData" => unimplemented!(),
        "undefined" => unimplemented!("'undefined' type deprecated"),
        "objectId" => unimplemented!(),
        "bool" => b.is_boolean(),
        "date" => unimplemented!(),
        "null" => b.is_null(),
        "regex" => unimplemented!(),
        "dbPointer" => unimplemented!("'dbPointer' type deprecated"),
        "javascript" => unimplemented!(),
        "symbol" => unimplemented!("'symbol' type deprecated"),
        "javascriptWithScope" => unimplemented!(),
        "int" => unimplemented!(),
        "timestamp" => unimplemented!(),
        "long" => b.is_i64(),
        "decimal" => unimplemented!(),
        "minKey" => unimplemented!(),
        "maxKey" => unimplemented!(),
        "number" => b.is_number(),
      })
    }
    Value::Number(a_num) => {
      let a_i64: i64 = from_value(a)?;
      Ok(match a_i64 {
        1 => b.is_f64(),
        2 => b.is_string(),
        3 => b.is_object(),
        4 => b.is_array(),
        5 => unimplemented!(),
        6 => unimplemented!("'undefined' type deprecated"),
        7 => unimplemented!(),
        8 => b.is_boolean(),
        9 => unimplemented!(),
        10 => b.is_null(),
        11 => unimplemented!(),
        12 => unimplemented!("'dbPointer' type deprecated"),
        13 => unimplemented!(),
        14 => unimplemented!("'symbol' type deprecated"),
        15 => unimplemented!(),
        16 => unimplemented!(),
        17 => unimplemented!(),
        18 => b.is_number(),
        19 => unimplemented!(),
        -1 => unimplemented!(),
        127 => unimplemented!(),
      })
    }
  }
}