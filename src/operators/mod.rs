use serde_json::Value;
use error::*;
use internal::{aggregate};
pub mod query;
pub mod aggregation;

pub fn evaluate_query(operation: &str, a: Value, b: Value, genealogy: Vec<String>) -> Result<bool> {
  match operation {
    // Comparison
    "eq" => query::comparison::eq(a, b, genealogy),
    "ne" => query::comparison::ne(a, b, genealogy),
    "gt" => query::comparison::gt(a, b, genealogy),
    "gte" => query::comparison::gte(a, b, genealogy),
    "lt" => query::comparison::lt(a, b, genealogy),
    "lte" => query::comparison::lte(a, b, genealogy),
    "in" => query::comparison::in_op(a, b, genealogy),
    "nin" => query::comparison::nin_op(a, b, genealogy),
    // Logical
    "and" => query::logical::and(a, b, genealogy),
    "not" => query::logical::not(a, b, genealogy),
    "nor" => query::logical::nor(a, b, genealogy),
    "or" => query::logical::or(a, b, genealogy),
    // Element
    "exists" => query::element::exists(a, b, genealogy),
    "type" => query::element::type_op(a, b, genealogy),
    // Evaluation
    "expr" => query::evaluation::expr(a, b, genealogy),
    "jsonSchema" => query::evaluation::json_schema(a, b, genealogy),
    "mod" => query::evaluation::mod_op(a, b, genealogy),
    "regex" => query::evaluation::regex_op(a, b, genealogy),
    "text" => query::evaluation::text(a, b, genealogy),
    "where" => query::evaluation::where_op(a, b, genealogy),
    // Geospatial
    "geoIntersects" => query::geospatial::geo_intersects(a, b, genealogy),
    "geoWithin" => query::geospatial::geo_within(a, b, genealogy),
    "near" => query::geospatial::near(a, b, genealogy),
    "nearSphere" => query::geospatial::near_sphere(a, b, genealogy),
    "box" => query::geospatial::box_op(a, b, genealogy),
    "center" => query::geospatial::center(a, b, genealogy),
    "centerSphere" => query::geospatial::center_sphere(a, b, genealogy),
    "geometry" => query::geospatial::geometry(a, b, genealogy),
    "maxDistance" => query::geospatial::max_distance(a, b, genealogy),
    "minDistance" => query::geospatial::min_distance(a, b, genealogy),
    "polygon" => query::geospatial::polygon(a, b, genealogy),
    "uniqueDocs" => query::geospatial::unique_docs(a, b, genealogy),
    // Array
    "all" => query::array::all(a, b, genealogy),
    "elemMatch" => query::array::elem_match(a, b, genealogy),
    "size" => query::array::size(a, b, genealogy),
    // Bitwise
    "bitsAllClear" => query::bitwise::bits_all_clear(a, b, genealogy),
    "bitsAllSet" => query::bitwise::bits_all_set(a, b, genealogy),
    "bitsAnyClear" => query::bitwise::bits_any_clear(a, b, genealogy),
    "bitsAnySet" => query::bitwise::bits_any_set(a, b, genealogy),
    // Projection
    // "elemMatch" => query::projection::elem_match(a, b, last_key),
    "meta" => query::projection::meta(a, b, genealogy),
    "slice" => query::projection::slice(a, b, genealogy),
    "" => query::projection::projection(a, b, genealogy),
  }
}

pub fn evaluate_aggregation(operation: &str, a: Value, context: Value, genealogy: Vec<String>) -> Result<Value> {  
  let a_aggregate = aggregate(a, genealogy)?;
  
  match operation {
    // Arithmetic
    "abs" => aggregation::arithmetic::abs(a_aggregate, genealogy),
    "add" => aggregation::arithmetic::add(a_aggregate, genealogy),
    "ceil" => aggregation::arithmetic::ceil(a_aggregate, genealogy),
    "divide" => aggregation::arithmetic::divide(a_aggregate, genealogy),
    "exp" => aggregation::arithmetic::exp(a_aggregate, genealogy),
    "floor" => aggregation::arithmetic::floor(a_aggregate, genealogy),
    "ln" => aggregation::arithmetic::ln(a_aggregate, genealogy),
    "log" => aggregation::arithmetic::log(a_aggregate, genealogy),
    "log10" => aggregation::arithmetic::log10(a_aggregate, genealogy),
    "mod" => aggregation::arithmetic::mod_op(a_aggregate, genealogy),
    "multiply" => aggregation::arithmetic::multiply(a_aggregate, genealogy),
    "pow" => aggregation::arithmetic::pow(a_aggregate, genealogy),
    "sqrt" => aggregation::arithmetic::sqrt(a_aggregate, genealogy),
    "subtract" => aggregation::arithmetic::subtract(a_aggregate, genealogy),
    "trunc" => aggregation::arithmetic::trunc(a_aggregate, genealogy),
    // // Array
    // "arrayElemAt" => aggregation::array::array_elem_at(a_aggregate),
    // "arrayToObject" => aggregation::array::array_to_object(a_aggregate),
    // "concatArrays" => aggregation::array::concat_arrays(a_aggregate),
    // "filter" => aggregation::array::filter(a_aggregate),
    // "in" => aggregation::array::in_op(a_aggregate),
    // "indexOfArray" => aggregation::array::index_of_array(a_aggregate),
    // "isArray" => aggregation::array::is_array(a_aggregate),
    // "map" => aggregation::array::map(a_aggregate),
    // "objectToArray" => aggregation::array::object_to_array(a_aggregate),
    // "range" => aggregation::array::range(a_aggregate),
    // _ => Err("Unknown aggregation operation"),
  }
}