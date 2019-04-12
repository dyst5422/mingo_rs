use error::*;
use operators;
use serde_json::{Map, Value};
use utils::resolve;

/// Test if an object matches a query
pub fn query(query_doc: &Value, object: &Value, genealogy: &Vec<String>) -> bool {
  match query_doc {
    Value::Object(query_obj) => query_obj.iter().all(|(key, query_value)| {
      let mut new_genealogy = genealogy.clone();
      new_genealogy.push(key.clone());
      match &key[..1] {
        "$" => match operators::evaluate_query(&key[1..], query_value, object, &new_genealogy) {
          Ok(val) => val,
          Err(_msg) => false,
        },
        _ => match resolve(key, object) {
          Some(val) => query(query_value, &val, &new_genealogy),
          None => false,
        },
      }
    }),
    _ => query_doc == object,
  }
}

/// Aggregate a value
pub fn aggregate(aggregation: &Value, context: &Value, genealogy: &Vec<String>) -> Result<Value> {
  match aggregation {
    Value::Object(aggregation_obj) => match aggregation_obj.len() {
      1 => {
        let (key, value) = aggregation_obj.iter().next().unwrap();
        let mut new_genealogy = genealogy.clone();
        new_genealogy.push(key.clone());
        let value_aggregate = aggregate(value, context, &new_genealogy)?;
        match &key[..1] {
          "$" => Ok(operators::evaluate_aggregation(
            &key[1..],
            &value_aggregate,
            context,
            &new_genealogy,
          )?),
          _ => Ok(value_aggregate),
        }
      },
      _ => {
        let mut aggregated_object = Map::new();
        for (key, value) in aggregation_obj.iter() {
          let mut new_genealogy = genealogy.clone();
          new_genealogy.push(key.clone());
          let value_aggregate = aggregate(value, context, &new_genealogy)?;
          aggregated_object.insert(
            key.to_owned(),
            value_aggregate,
          );
        }
        Ok(aggregated_object.into())
      }
    },
    Value::Array(aggregation_value) => aggregation_value
      .iter()
      .enumerate()
      .map(|(ind, value)| {
        let mut new_genealogy = genealogy.clone();
        new_genealogy.push(ind.to_string());
        aggregate(value, context, &new_genealogy)
      })
      .collect::<Result<Value>>(),
    _ => Ok(aggregation.clone()),
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use serde_json::json;
//   // #[test]
//   // fn name() {
//   //   let q = json!({ "key": { "$exists": true } });
//   //   let d = json!({ "key": 5 });
//   //   println!("{:?}", q);
//   //   println!("{:?}", d);

//   //   // assert_eq!(
//   //   //   query(
//   //   //     &q,
//   //   //     &d,
//   //   //     &vec![]
//   //   //   ),
//   //   //   true,
//   //   // );
//   // }

//   // #[test]
//   // fn aggregate_test() {
//   //   let q = json!({ "$add": [1, 2] });
//   //   let d = json!({ });

//   //   println!("{:?}", aggregate(&q, &d, &vec![]))

//   //   // assert_eq!(
//   //   //   query(
//   //   //     &q,
//   //   //     &d,
//   //   //     &vec![]
//   //   //   ),
//   //   //   true,
//   //   // );
//   // }
// }
