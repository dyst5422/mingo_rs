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

#[test]
fn concat_arrays() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$concatArrays": [[1,2], [3,4]] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    json!([1,2,3,4])
  );

  assert_eq!(
    aggregate(
      &json!({ "$concatArrays": [[1,[1,2]], [3,4]] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    json!([1,[1,2],3,4])
  );
}

#[test]
fn filter() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$filter": { "input": [1,2,3], "cond": true } }),
      &json!({}),
      &vec![]
    ).unwrap(),
    json!([1, 2, 3])
  );

  assert_eq!(
    aggregate(
      &json!({ "$filter": { "input": [1,2,3], "cond": false } }),
      &json!({}),
      &vec![]
    ).unwrap(),
    json!([])
  );
}

#[test]
fn in_op() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$in": [1, [1,2,3]] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    true
  );

  assert_eq!(
    aggregate(
      &json!({ "$in": [4, [1,2,3]] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    false
  );
}

#[test]
fn index_of_array() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$indexOfArray": [[0, 1, 2, 3], 2, 0, 3] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0
  );

  assert_eq!(
    aggregate(
      &json!({ "$indexOfArray": [[0, 1, 2, 3], 2, 0, 2] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    -1.0
  );

  assert_eq!(
    aggregate(
      &json!({ "$indexOfArray": [[0, 1, 2, 3], 2, 0] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0
  );

  assert_eq!(
    aggregate(
      &json!({ "$indexOfArray": [[0, 1, 2, 3], 2] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0
  );
}

#[test]
fn is_array() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$isArray": [0, 1, 2, 3] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    true
  );

  assert_eq!(
    aggregate(
      &json!({ "$isArray": 1 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    false
  );
}

#[test]
fn map() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$map": { "input": [1, 2, 3], "in": { "$add": ["$$val", 1] }, "as": "val"} }),
      &json!({}),
      &vec![]
    ).unwrap(),
    true
  );

}

