use serde_json;
use internal;
pub mod query;
pub mod aggregation;


pub fn evaluate_query(operation: &str, a: &serde_json::Value, b: &serde_json::Value, last_key: Option<&str>) -> Result<bool, String> {
  match operation {
    // Comparison
    "eq" => query::comparison::eq(a, b, last_key),
    "ne" => query::comparison::ne(a, b, last_key),
    "gt" => query::comparison::gt(a, b, last_key),
    "gte" => query::comparison::gte(a, b, last_key),
    "lt" => query::comparison::lt(a, b, last_key),
    "lte" => query::comparison::lte(a, b, last_key),
    "in" => query::comparison::in_op(a, b, last_key),
    "nin" => query::comparison::nin_op(a, b, last_key),
    // Logical
    "and" => query::logical::and(a, b, last_key),
    "not" => query::logical::not(a, b, last_key),
    "nor" => query::logical::nor(a, b, last_key),
    "or" => query::logical::or(a, b, last_key),
    // Element
    "exists" => query::element::exists(a, b, last_key),
    "type" => query::element::type_op(a, b, last_key),
    // Evaluation
    "expr" => query::evaluation::expr(a, b, last_key),
    "jsonSchema" => query::evaluation::json_schema(a, b, last_key),
    "mod" => query::evaluation::mod_op(a, b, last_key),
    "regex" => query::evaluation::regex_op(a, b, last_key),
    "text" => query::evaluation::text(a, b, last_key),
    "where" => query::evaluation::where_op(a, b, last_key),
    // Geospatial
    "geoIntersects" => query::geospatial::geo_intersects(a, b, last_key),
    "geoWithin" => query::geospatial::geo_within(a, b, last_key),
    "near" => query::geospatial::near(a, b, last_key),
    "nearSphere" => query::geospatial::near_sphere(a, b, last_key),
    "box" => query::geospatial::box_op(a, b, last_key),
    "center" => query::geospatial::center(a, b, last_key),
    "centerSphere" => query::geospatial::center_sphere(a, b, last_key),
    "geometry" => query::geospatial::geometry(a, b, last_key),
    "maxDistance" => query::geospatial::max_distance(a, b, last_key),
    "minDistance" => query::geospatial::min_distance(a, b, last_key),
    "polygon" => query::geospatial::polygon(a, b, last_key),
    "uniqueDocs" => query::geospatial::unique_docs(a, b, last_key),
    // Array
    "all" => query::array::all(a, b, last_key),
    "elemMatch" => query::array::elem_match(a, b, last_key),
    "size" => query::array::size(a, b, last_key),
    // Bitwise
    "bitsAllClear" => query::bitwise::bits_all_clear(a, b, last_key),
    "bitsAllSet" => query::bitwise::bits_all_set(a, b, last_key),
    "bitsAnyClear" => query::bitwise::bits_any_clear(a, b, last_key),
    "bitsAnySet" => query::bitwise::bits_any_set(a, b, last_key),
    // Projection
    // "elemMatch" => query::projection::elem_match(a, b, last_key),
    "meta" => query::projection::meta(a, b, last_key),
    "slice" => query::projection::slice(a, b, last_key),
    "" => query::projection::projection(a, b, last_key),
    _ => Err("Unknown query operation".to_string()),
  }
}

pub fn evaluate_aggregation(operation: &str, a: &serde_json::Value, context: &serde_json::Value) -> Result<serde_json::Value, String> {
  let a_aggregate = &internal::aggregate(a, context)?;
  
  match operation {
    // Arithmetic
    "abs" => aggregation::arithmetic::abs(a_aggregate),
    "add" => aggregation::arithmetic::add(a_aggregate),
    "ceil" => aggregation::arithmetic::ceil(a_aggregate),
    _ => Err("Unknown aggregation operation".to_string()),
  }
}