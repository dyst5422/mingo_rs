use serde_json::{Value, Map};
use utils::from_value;
use error::*;

pub fn exists(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("genealogy: {:?}", genealogy);
    let a_bool: bool = from_value(a)?;
    println!("a_bool: {:?}", a_bool);
    let b_obj: Map<String, Value> = from_value(b)?;
    println!("b_obj: {:?}", b_obj);
    let mut reversed = genealogy.iter().rev();
    println!("reversed: {:?}", reversed);
    let last = reversed.next().unwrap();
    let second_from_last = reversed.next().unwrap();
    println!("last: {:?}", last);
    println!("second_from_last: {:?}", second_from_last);
    Ok(a_bool == b_obj.contains_key(second_from_last))
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use serde_json::json;
//   #[test]
//   fn name() {
//     let val = exists(&json!(true), &json!({ "key": 5 }), &vec!["key".to_string()]);
//     match val {
//       Ok(val) => assert_eq!(val, true),
//       Err(err) => panic!(err)
//     };
//   }
// }


pub fn type_op(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  match a {
    Value::String(a_str) => {
      match a_str.as_str() {
        "double" => Ok(b.is_f64()),
        "string" => Ok(b.is_string()),
        "object" => Ok(b.is_object()),
        "array" => Ok(b.is_array()),
        "binData" => unimplemented!(),
        "undefined" => unimplemented!("'undefined' type deprecated"),
        "objectId" => unimplemented!(),
        "bool" => Ok(b.is_boolean()),
        "date" => unimplemented!(),
        "null" => Ok(b.is_null()),
        "regex" => unimplemented!(),
        "dbPointer" => unimplemented!("'dbPointer' type deprecated"),
        "javascript" => unimplemented!(),
        "symbol" => unimplemented!("'symbol' type deprecated"),
        "javascriptWithScope" => unimplemented!(),
        "int" => unimplemented!(),
        "timestamp" => unimplemented!(),
        "long" => Ok(b.is_i64()),
        "decimal" => unimplemented!(),
        "minKey" => unimplemented!(),
        "maxKey" => unimplemented!(),
        "number" => Ok(b.is_number()),
        _ => Err(MingoError::UnknownTypeOperationValueError {
          message: "",
          genealogy: genealogy.join(","),
          argument: a.to_owned(),
        }.into()),
      }
    }
    Value::Number(_) => {
      let a_i64: i64 = from_value(a)?;
      match a_i64 {
        1 => Ok(b.is_f64()),
        2 => Ok(b.is_string()),
        3 => Ok(b.is_object()),
        4 => Ok(b.is_array()),
        5 => unimplemented!(),
        6 => unimplemented!("'undefined' type deprecated"),
        7 => unimplemented!(),
        8 => Ok(b.is_boolean()),
        9 => unimplemented!(),
        10 => Ok(b.is_null()),
        11 => unimplemented!(),
        12 => unimplemented!("'dbPointer' type deprecated"),
        13 => unimplemented!(),
        14 => unimplemented!("'symbol' type deprecated"),
        15 => unimplemented!(),
        16 => unimplemented!(),
        17 => unimplemented!(),
        18 => Ok(b.is_number()),
        19 => unimplemented!(),
        -1 => unimplemented!(),
        127 => unimplemented!(),
        _ => Err(MingoError::UnknownTypeOperationValueError {
          message: "",
          genealogy: genealogy.join(","),
          argument: a.to_owned(),
        }.into()),
      }
    },
    _ => Err(MingoError::UnknownTypeOperationValueError {
      message: "Can only take string or integer arguments",
      genealogy: genealogy.join(","),
      argument: a.to_owned(),
    }.into()),
  }
}