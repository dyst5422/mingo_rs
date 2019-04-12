extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::query;
use serde_json::json;


// #[test]
// fn exists() -> () {
//   // assert_eq!(
//   //   query(
//   //     &json!({"key": { "$exists": true } }),
//   //     &json!({ "key": 5 }),
//   //     &vec![]
//   //   ),
//   //   true
//   // );
//   // assert_eq!(
//   //   query(
//   //     &json!({"key": { "$exists": true } }),
//   //     &json!({ "key1": 6 }),
//   //     &vec![]
//   //   ),
//   //   false
//   // );
//   // assert_eq!(
//   //   query(
//   //     &json!({"key": { "$exists": false } }),
//   //     &json!({ "key1": 4 }),
//   //     &vec![]
//   //   ),
//   //   true
//   // );
//   // assert_eq!(
//   //   query(
//   //     &json!({"key": { "$exists": false } }),
//   //     &json!({ "key": 6 }),
//   //     &vec![]
//   //   ),
//   //   false
//   // );
// }

#[test]
fn type_op() {
  assert_eq!(
    query(
      &json!({"key": { "$type": "number" } }),
      &json!({ "key": 6 }),
      &vec![]
    ),
    true
  );
  assert_eq!(
    query(
      &json!({"key": { "$type": "string" } }),
      &json!({ "key": 6 }),
      &vec![]
    ),
    false
  );
}
