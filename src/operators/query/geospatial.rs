use serde_json::{Value};
use error::*;

pub fn geo_intersects(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn geo_within(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn near(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn near_sphere(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn box_op(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn center(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn center_sphere(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn geometry(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn max_distance(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn min_distance(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn polygon(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}

pub fn unique_docs(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}