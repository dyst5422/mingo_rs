use serde_json::{Value};
use error::*;

pub fn projection(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn elem_match(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn meta(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn slice(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}