use operators;
use serde_json;

/// Test if an object matches a query
pub fn query(query_doc: &serde_json::Value, object: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  match query_doc {
    serde_json::Value::Object(query_obj) => Ok(query_obj.iter().all(|(key, query_value)| match &key[..1] {
      "$" => match operators::evaluate_query(&key[1..], query_value, object, last_key) {
        Ok(val) => val,
        Err(_msg) => false
      },
      _ => match resolve(key, object) {
        Ok(val) => match query(query_value, val, Some(key)) {
          Ok(test_val) => test_val,
          Err(_msg) => false,
        },
        Err(_msg) => false,
      },
    })),
    // serde_json::Value::Object(query_obj) => Ok(false),
    serde_json::Value::Array(query_value) => match object {
      serde_json::Value::Array(object_value) => Ok(query_value == object_value),
      _ => Ok(false),
    },
    serde_json::Value::Number(query_value) => match object {
      serde_json::Value::Number(object_value) => Ok(query_value == object_value),
      _ => Ok(false),
    },
    serde_json::Value::String(query_value) => match object {
      serde_json::Value::String(object_value) => Ok(query_value == object_value),
      _ => Ok(false),
    },
    serde_json::Value::Bool(query_value) => match object {
      serde_json::Value::Bool(object_value) => Ok(query_value == object_value),
      _ => Ok(false),
    },
    serde_json::Value::Null => match object {
      serde_json::Value::Null => Ok(true),
      _ => Ok(false),
    },
  }
}

/// Test if an object matches a query
pub fn aggregate(aggregation: &serde_json::Value, context: &serde_json::Value) -> Result<serde_json::Value, String> {
  match aggregation {
    serde_json::Value::Object(aggregation_obj) => match aggregation_obj.len() {
      1 => {
        let keys: Vec<&String> = aggregation_obj.keys().collect();
        let first_key = keys[0];
        match aggregation_obj.get(first_key) {
          Some(aggregation_value) => match &first_key[..1] {
            "$" => match operators::evaluate_aggregation(&first_key[1..], aggregation_value, context) {
              Ok(val) => Ok(val),
              Err(msg) => Err(msg),
            },
            _ => Err("Aggregations require an operation".to_string()),
          },
          None => Err("Aggregations require an operation".to_string()),
        }
      },
      _ => Err("Aggregations can only have one operation".to_string()),
    },
    serde_json::Value::Array(aggregation_value) =>  aggregation_value.iter().map(|x| aggregate(x, context)).collect(),
    serde_json::Value::Number(aggregation_value) => match serde_json::to_value(aggregation_value.clone()) {
      Ok(val) => Ok(val),
      Err(_err) => Err("Cannot convert to serde Value".to_string()),
    },
    serde_json::Value::String(aggregation_value) => match &aggregation_value[..1] {
      "$" => match resolve(&aggregation_value[1..], context) {
        Ok(val) => Ok(val.clone()),
        Err(msg) => Err(msg),
      },
      _ => match serde_json::to_value(aggregation_value.clone()) {
        Ok(val) => Ok(val),
        Err(_err) => Err("Cannot convert to serde Value".to_string()),
      },
    },
    serde_json::Value::Bool(aggregation_value) => match serde_json::to_value(aggregation_value.clone()) {
      Ok(val) => Ok(val),
      Err(_err) => Err("Cannot convert to serde Value".to_string()),
    },
    serde_json::Value::Null => match serde_json::to_value(serde_json::Value::Null) {
      Ok(val) => Ok(val),
      Err(_err) => Err("Cannot convert to serde Value".to_string()),
    },
  }
}

/// Resolve a value at a path in a document
fn resolve<'a>(
  path: &str,
  object: &'a serde_json::Value,
) -> Result<&'a serde_json::Value, String> {
  let path_parts: Vec<&str> = path.splitn(2, '.').collect();

  match object.get(path_parts[0]) {
    Some(sub_object) => match path_parts.len() {
      2 => return resolve(path_parts[1], sub_object),
      1 => return Ok(sub_object),
      _ => unreachable!(),
    },
    None => return Err("Tried accessing a path on document that does not exist".to_string()),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn query_test() -> () {
    match query(&json!({ "key": { "$elemMatch": { "$gt": 1, "$lt": 3 } } }), &json!({ "key": [2] }), None) {
      Ok(val) => val,
      Err(msg) => {
        println!("{}", msg);
        panic!(msg);
      },
    };
  }
  #[test]
  fn aggregate_test() -> () {
    match aggregate(&json!({ "$abs": -1 }), &json!({})) {
      Ok(val) => val,
      Err(msg) => {
        println!("{}", msg);
        panic!(msg);
      },
    };
  }
}