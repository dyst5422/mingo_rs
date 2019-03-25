use operators;
use serde_json::Value;
use error::*;

/// Test if an object matches a query
pub fn query(query_doc: Value, object: Value, genealogy: Vec<String>) -> bool {
  match query_doc {
    Value::Object(query_obj) => query_obj.iter().all(|(key, &query_value)| {
      let new_genealogy = genealogy.to_vec();
      new_genealogy.push(key.clone());
      match &key[..1] {
        "$" => match operators::evaluate_query(&key[1..], query_value, object, new_genealogy) {
          Ok(val) => val,
          Err(_msg) => false,
        },
        _ => match resolve(key, object) {
          Some(val) => query(query_value, val, new_genealogy),
          None => false,
        },
      }
    }),
    _ => query_doc == object,
  }
}

/// Test if an object matches a query
pub fn aggregate(aggregation: Value, context: Value, genealogy: Vec<String>) -> Result<Value> {
  match aggregation {
    Value::Object(aggregation_obj) => match aggregation_obj.len() {
      1 => {
        let entries: Vec<(&String, &Value)> = aggregation_obj.iter().collect();
        let (key, value) = entries[0];
        let new_genealogy = genealogy.clone();
        new_genealogy.push(key.clone());
        let value_aggregate = aggregate(aggregation, context, new_genealogy)?;
        match &key[..1] {
          "$" => operators::evaluate_aggregation(&key[1..], value_aggregate, context, new_genealogy)
            .chain_err(|| ErrorKind::MingoError(new_genealogy)),
          _ => Err(ErrorKind::MingoError(new_genealogy).into()),
        }
      },
      _ => Err(ErrorKind::MingoError(genealogy).into()),
    },
    Value::Array(aggregation_value) =>  aggregation_value.iter().enumerate().map(|(ind, &x)| {
      let new_genealogy = genealogy.clone();
      new_genealogy.push(ind.to_string());
      aggregate(x, context, new_genealogy)
    }).collect(),
    _ => Ok(aggregation),
  }
}

/// Resolve a value at a path in a document
fn resolve(
  path: &str,
  object: Value,
) -> Option<Value> {
  let path_parts: Vec<&str> = path.splitn(2, '.').collect();

  match object.get(path_parts[0]) {
    Some(&sub_object) => match path_parts.len() {
      2 => resolve(path_parts[1], sub_object),
      1 => Some(sub_object),
      _ => unreachable!(),
    }
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