extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::query;
use serde_json::json;

#[test]
fn and() -> () {
  assert_eq!(
    query(
      &json!({ "$and": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 6 } }] }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "$and": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 6 } }] }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    false
  );
  assert_eq!(
    query(
      &json!({ "$and": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 6 } }] }),
      &json!({ "key": 6 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn not() -> () {
  assert_eq!(
    query(
      &json!({ "$not": { "key": { "$gt": 4 } } }),
      &json!({ "key": 4 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "$not": { "key": { "$gt": 4 } } }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn nor() -> () {
  assert_eq!(
    query(
      &json!({ "$nor": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 2 } }] }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "$nor": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 2 } }] }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    false
  );
  assert_eq!(
    query(
      &json!({ "$nor": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 2 } }] }),
      &json!({ "key": 1 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn or() -> () {
  assert_eq!(
    query(
      &json!({ "$or": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 2 } }] }),
      &json!({ "key": 3 }),
      &vec![]
    ),
    false
  );
  assert_eq!(
    query(
      &json!({ "$or": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 2 } }] }),
      &json!({ "key": 5 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "$or": [{ "key": { "$gt": 4 } }, { "key": { "$lt": 2 } }] }),
      &json!({ "key": 1 }),
      &vec![]
    ),
    true
  );
}