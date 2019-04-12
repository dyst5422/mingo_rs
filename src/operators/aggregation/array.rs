use error::*;
use external::aggregate;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Map, Value};
use utils::from_value;
use failure;

pub fn array_elem_at(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  let (first_vec, second_usize): (Vec<Value>, usize) = from_value(a)?;
  Ok(first_vec[second_usize].clone())
}

pub fn array_to_object(a: &Value, _context: &Value, genealogy: &Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  struct KVMap {
    k: String,
    v: Value,
  }
  type KVTuple = (String, Value);

  let a_vec: Vec<Value> = from_value(a)?;

  let solution: std::result::Result<Map<String, Value>, failure::Error> = a_vec.iter().try_fold(Map::new(), |mut accum, item| {
    match item {
      serde_json::Value::Object(_) => {
        let kv_map: KVMap = from_value(item)?;
        accum.insert(kv_map.k.clone(), kv_map.v.clone());
        Ok(accum)
      },
      serde_json::Value::Array(_) => {
        let kv_tuple: KVTuple = from_value(item)?;
        accum.insert(kv_tuple.0.clone(), kv_tuple.1.clone());
        Ok(accum)
      }
      _ => Err(MingoError::ImproperlyFormattedQueryError{
        genealogy: genealogy.join(","),
        message: "arrayToObject entries must either be objects like { 'k': string, 'v': value } or tuples [k, v]"
      }.into()),
    }
  });

  Ok(solution?.into())
}

pub fn concat_arrays(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  let a_vec: Vec<Vec<Value>> = from_value(a)?;
  let merged_vec: Vec<Value> = a_vec
    .iter()
    .flat_map(|x| x.iter())
    .map(|x| x.clone())
    .collect();
  Ok(merged_vec.into())
}

#[derive(Serialize, Deserialize, Clone)]
enum AsEnum {
  As(String),
  None,
}

impl Into<String> for AsEnum {
  fn into(self) -> String {
    match self {
      AsEnum::As(as_key) => as_key,
      AsEnum::None => "this".to_string(),
    }
  }
}

pub fn filter(a: &Value, _context: &Value, genealogy: &Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  struct FilterDefinition {
    input: Vec<Value>,
    cond: Value,
    r#as: AsEnum,
  }
  let a_filter: FilterDefinition = from_value(a)?;

  a_filter
    .input
    .iter()
    .filter_map(|x| {
      let as_key: String = a_filter.r#as.clone().into();
      let mut context = Map::new();
      context.insert(as_key, x.clone());
      aggregate(&a_filter.cond, &context.into(), genealogy).into()
    })
    .collect()
}

pub fn in_op(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  let (a_val, b_vec): (Value, Vec<Value>) = from_value(a)?;
  Ok(b_vec.contains(&a_val).into())
}

pub fn index_of_array(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  enum IndexOfArrayInputs {
    Two((Vec<Value>, Value)),
    Three((Vec<Value>, Value, usize)),
    Four((Vec<Value>, Value, usize, usize)),
  }
  #[derive(Serialize, Deserialize)]
  struct IndexOfArrayDefinition {
    vector: Vec<Value>,
    value: Value,
    start_index: usize,
    end_index: usize,
  }

  impl Into<IndexOfArrayDefinition> for IndexOfArrayInputs {
    fn into(self) -> IndexOfArrayDefinition {
      match self {
        IndexOfArrayInputs::Two((vec, val)) => IndexOfArrayDefinition {
          end_index: vec.len(),
          vector: vec,
          value: val,
          start_index: 0,
        },
        IndexOfArrayInputs::Three((vec, val, start_index)) => IndexOfArrayDefinition {
          end_index: vec.len(),
          vector: vec,
          value: val,
          start_index: start_index,
        },
        IndexOfArrayInputs::Four((vec, val, start_index, end_index)) => IndexOfArrayDefinition {
          vector: vec,
          value: val,
          start_index: start_index,
          end_index: end_index,
        },
      }
    }
  }

  let index_of_array_def: IndexOfArrayDefinition = from_value(a)?;

  Ok(
    match index_of_array_def.vector[index_of_array_def.start_index..index_of_array_def.end_index]
      .iter()
      .position(|item| item == &index_of_array_def.value)
    {
      Some(index) => index.into(),
      None => (-1).into(),
    },
  )
}

pub fn is_array(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  Ok(match a {
    Value::Array(..) => true.into(),
    _ => false.into(),
  })
}

pub fn map(a: &Value, _context: &Value, genealogy: &Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  struct MapDefinition {
    input: Vec<Value>,
    r#in: Value,
    r#as: AsEnum,
  }

  let a_map: MapDefinition = from_value(a)?;

  a_map
    .input
    .iter()
    .map(|x| {
      let as_key: String = a_map.r#as.clone().into();
      let mut context = Map::new();
      context.insert(as_key, x.clone());
      aggregate(&a_map.r#in, &context.into(), genealogy)
    })
    .collect()
}

pub fn object_to_array(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  let a_map: Map<String, Value> = from_value(a)?;

  #[derive(Serialize, Deserialize)]
  struct MappedKV {
    k: String,
    v: Value,
  }

  let a_vec: Vec<Map<String, Value>> = a_map
    .iter()
    .map(|(key, value)| {
      let mut kv_map = Map::new();
      kv_map.insert("k".into(), key.clone().into());
      kv_map.insert("v".into(), value.clone());
      kv_map
    })
    .collect();
  Ok(a_vec.into())
}

pub fn range(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  enum RangeInputs {
    Two((i64, i64)),
    Three((i64, i64, usize)),
  }
  #[derive(Serialize, Deserialize)]
  struct RangeDefinition {
    start: i64,
    end: i64,
    step_size: usize,
  }

  impl Into<RangeDefinition> for RangeInputs {
    fn into(self) -> RangeDefinition {
      match self {
        RangeInputs::Two((start, end)) => RangeDefinition {
          start,
          end,
          step_size: 1,
        },
        RangeInputs::Three((start, end, step_size)) => RangeDefinition {
          start,
          end,
          step_size,
        },
      }
    }
  }

  let range_def: RangeDefinition = from_value(a)?;

  let range_iterator: Vec<i64> = (range_def.start..range_def.end)
    .step_by(range_def.step_size)
    .collect();
  Ok(range_iterator.into())
}
