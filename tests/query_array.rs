extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::query;
use serde_json::json;

#[test]
fn all() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$all": [1,2,3] } }),
      &json!({ "key": [0,1,2,3,4] }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$all": [1,2,3] } }),
      &json!({ "key": [0,1,3,4] }),
      &vec![]
    ),
    false
  );
}

#[test]
fn elem_match() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$elemMatch": 1 } }),
      &json!({ "key": [0,1] }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$elemMatch": 1 } }),
      &json!({ "key": [0,3] }),
      &vec![]
    ),
    false
  );
}

#[test]
fn size() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$size": 1 } }),
      &json!({ "key": [0] }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({ "key": { "$size": 1 } }),
      &json!({ "key": [0,3] }),
      &vec![]
    ),
    false
  );
}
