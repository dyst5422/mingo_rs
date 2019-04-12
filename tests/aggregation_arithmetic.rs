extern crate mingo_rs;
extern crate serde_json;

use mingo_rs::aggregate;
use serde_json::json;

#[test]
fn abs() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$abs": -1.0 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    1.0
  );
}

#[test]
fn add() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$add": [1, 2] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    3.0
  );
}

#[test]
fn ceil() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$ceil": 1.5 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0
  );
}

#[test]
fn divide() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$divide": [1, 2] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    0.5
  );
}

#[test]
fn exp() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$exp": 1 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    1.0_f64.exp()
  );
}

#[test]
fn floor() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$floor": 1.5 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    1.0
  );
}

#[test]
fn ln() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$ln": 2 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0_f64.ln()
  );
}

#[test]
fn log() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$log": [3,2] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    3.0_f64.log(2.0)
  );
}

#[test]
fn log10() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$log10": 1 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    0.0
  );
}

#[test]
fn mod_op() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$mod": [12, 5] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0
  );
}

#[test]
fn multiply() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$multiply": [2, 6] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    12.0
  );
}

#[test]
fn pow() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$pow": [2, 3] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    8.0
  );
}

#[test]
fn sqrt() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$sqrt": 4 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    2.0
  );
}

#[test]
fn subtract() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$subtract": [4, 1] }),
      &json!({}),
      &vec![]
    ).unwrap(),
    3.0
  );
}

#[test]
fn trunc() -> () {
  assert_eq!(
    aggregate(
      &json!({ "$trunc": -2.1 }),
      &json!({}),
      &vec![]
    ).unwrap(),
    -2.0
  );
}
