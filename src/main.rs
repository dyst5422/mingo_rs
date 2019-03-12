#[macro_use]
extern crate serde_json;
extern crate mingo_rs;

fn main() {
  println!(
    "{:?}",
    mingo_rs::internal::query(&json!({ "key": { "$elemMatch": { "$gt": 1, "$lt": 3 } } }), &json!({ "key": [2] }), None)
  );
  println!(
    "{:?}",
    mingo_rs::internal::aggregate(&json!({ "$add": [-1, "$val" ] }), &json!({ "val": -1 }))
  );
}

