extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::query;
use serde_json::json;

#[test]
fn expr() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$expr": { "$add": [2,2] } } }),
      &json!({ "key": 4.0 }),
      &vec![]
    ),
    true
  );
}

#[test]
fn json_schema() -> () {
  assert_eq!(
    query(
      &json!({ "$jsonSchema": {
        "$id": "https://example.com/person.schema.json",
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Person",
        "type": "object",
        "properties": {
          "firstName": {
            "type": "string",
            "description": "The person's first name."
          },
          "lastName": {
            "type": "string",
            "description": "The person's last name."
          },
          "age": {
            "description": "Age in years which must be equal to or greater than zero.",
            "type": "integer",
            "minimum": 0
          }
        }
      } }),
      &json!({
        "firstName": "John",
        "lastName": "Doe",
        "age": 21
      }),
      &vec![]
    ),
    true
  );
}

#[test]
fn mod_op() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$mod": [4, 0] } }),
      &json!({ "key": 12 }),
      &vec![]
    ),
    true
  );

  assert_eq!(
    query(
      &json!({ "key": { "$mod": [4, 0] } }),
      &json!({ "key": 11 }),
      &vec![]
    ),
    false
  );
}

#[test]
fn regex_op() -> () {
  assert_eq!(
    query(
      &json!({ "key": { "$regex": "some.*val" } }),
      &json!({ "key": "some45val" }),
      &vec![]
    ),
    true
  );

  assert_eq!(
    query(
      &json!({ "key": { "$regex": "some.*val" } }),
      &json!({ "key": "some45al" }),
      &vec![]
    ),
    false
  );
}
