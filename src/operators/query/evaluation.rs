use serde_json::Value;
use regex;
use external::{aggregate, query};
use utils::from_value;
use error::*;
use valico::json_schema;
// use tantivy::schema::{Schema, TEXT};
// use tantivy::doc;
// use tantivy::Index;

pub fn expr(a: &Value, b: &Value, genealogy: &Vec<String>) -> Result<bool> {
  let a_aggregate = aggregate(a, b, genealogy)?;
  Ok(query(&a_aggregate, b, genealogy))
}

pub fn json_schema(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let mut scope = json_schema::Scope::new();
  let schema = scope.compile_and_return(a.clone(), true).unwrap();
  Ok(schema.validate(b).is_valid())
}

pub fn mod_op(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let (divisor, remainder): (f64, f64) = from_value(a)?;
  let b_f64: f64 = from_value(b)?;

  Ok(remainder == b_f64 % divisor)
}

pub fn regex_op(a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  let a_str: String = from_value(a)?;
  let b_str: String = from_value(b)?;

  Ok(regex::Regex::new(&a_str)?.is_match(&b_str))
}

pub fn text(_a: &Value, b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
  // let mut schema_builder = Schema::builder();
  // let text = schema_builder.add_text_field("text", TEXT);
  // let schema = schema_builder.build();

  // let b_string: String = from_value(b)?;

  // let document = doc!(
  //   text => b_string,
  // );

  // let index = Index::create_in_ram(schema);

  // match index.writer(100_000_000) {
  //   Ok(index_writer) => {
  //     index_writer.add_document(document);
  //     index_writer.commit()?;

  //   },
  //   Err(err) => Err(err.into())
  // }




  // unimplemented!()
}

pub fn where_op(_a: &Value, _b: &Value, _genealogy: &Vec<String>) -> Result<bool> {
  unimplemented!()
}
