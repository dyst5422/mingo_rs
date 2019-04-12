extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::query;
use serde_json::json;

#[test]
fn eq() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$eq": 4 } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$eq": 4 } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn ne() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$ne": 4 } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    false
  );
  assert_eq!(
    query(
      &json!({ "key": { "$ne": 4 } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    true
  );
}

#[test]
fn gt() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$gt": 4 } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$gt": 4 } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn gte() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$gte": 4 } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$gte": 4 } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$gte": 4 } }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn lt() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$lt": 4 } }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$lt": 4 } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn lte() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$lte": 4 } }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$lte": 4 } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$lte": 4 } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn in_op() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$in": [1,2,3,4] } }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$in": [1,2,3,4] } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn nin_op() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$nin": [1,2,3,4] } }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    false
  );
  assert_eq!(
    query(
      &json!({ "key": { "$nin": [1,2,3,4] } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    true
  );
}