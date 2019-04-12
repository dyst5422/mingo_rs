use serde_json;

pub fn from_value<T: serde::de::DeserializeOwned>(
  value: &serde_json::Value,
) -> std::result::Result<T, serde_json::Error> {
  T::deserialize(value.clone())
}

/// Resolve a value at a path in a document
pub fn resolve(path: &str, object: &serde_json::Value) -> Option<serde_json::Value> {
  let path_parts: Vec<&str> = path.splitn(2, '.').collect();

  match object.get(path_parts[0]) {
    Some(sub_object) => match path_parts.len() {
      2 => resolve(path_parts[1], sub_object),
      1 => Some(sub_object.clone()),
      _ => unreachable!(),
    },
    None => None,
  }
}