extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::aggregate;
use serde_json::json;

#[test]
fn array_elem_at() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$arrayElemAt": [[0, 1, 2], 1] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    1.0
  );
}

#[test]
fn array_to_object() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$arrayToObject": [{"k": "key0", "v": 0}, {"k": "key1", "v": 1}, {"k": "key2", "v": 2}] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    json!({"key0": 0, "key1": 1, "key2": 2})
  );

  assert_eq!(
    aggregate(
      &json!({ "$arrayToObject": [["key0", 0], ["key1", 1], ["key2", 2]] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    json!({"key0": 0, "key1": 1, "key2": 2})
  );
}
