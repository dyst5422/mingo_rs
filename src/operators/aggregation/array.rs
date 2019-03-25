use serde_json::{Value, from_value, Map};
#[macro_use]
use serde_derive::{Serialize, Deserialize};
use error::*;
use internal::{aggregate};

pub fn array_elem_at(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (first_vec, second_usize): (Vec<Value>, usize) = from_value(a)?;
  Ok(first_vec[second_usize])
}

pub fn array_to_object(a: Value, genealogy: Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  struct KVMap {
    k: String,
    v: Value,
  }
  type KVTuple = (String, Value);
  #[derive(Serialize, Deserialize)]
  enum KV {
    Map(KVMap),
    Tuple(KVTuple),
  }
  let a_vec: Vec<KV> = from_value(a)?;
  Ok(a_vec.iter()
    .fold(Map::new(), |mut accum, item| match item {
      KV::Map(kv_map) => {
        accum.insert(kv_map.k, kv_map.v);
        accum
      },
      KV::Tuple(kv_tuple) => {
        accum.insert(kv_tuple.0, kv_tuple.1);
        accum
      }
    }).into())
}

pub fn concat_arrays(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_vec: Vec<Vec<Value>> = from_value(a)?;
  let merged_vec: Vec<Value> = a_vec.iter().flat_map(|&x| x.iter()).map(|&x| x).collect();
  Ok(merged_vec.into())
}

#[derive(Serialize, Deserialize)]
enum AsEnum {
  As(String),
  None,
}

impl Into<String> for AsEnum {
  fn into(asEnum: AsEnum) -> String {
    match asEnum {
      AsEnum::As(asKey) => asKey,
      AsEnum::None => "this".to_string(),
    }
  }
}

pub fn filter(a: Value, genealogy: Vec<String>) -> Result<Value> {

  #[derive(Serialize, Deserialize)]
  struct FilterDefinition {
    input: Vec<Value>,
    cond: Value,
    r#as: AsEnum,
  }
  let a_filter: FilterDefinition = from_value(a)?;

  a_filter.input.iter().filter_map(|&x| {
    let asKey: String = a_filter.r#as.into();
    let mut context = Map::new();
    context.insert(asKey, x);
    aggregate(a_filter.cond, context.into(), genealogy).into()
  }).collect()
}

pub fn in_op(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let (a_val, b_vec): (Value, Vec<Value>) = from_value(a)?;
  Ok(b_vec.contains(&a_val).into())
}

pub fn index_of_array(a: Value, genealogy: Vec<String>) -> Result<Value> {
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
    fn into(indexOfArrayInputs: IndexOfArrayInputs) -> IndexOfArrayDefinition {
      match indexOfArrayInputs {
        IndexOfArrayInputs::Two((vec, val)) => IndexOfArrayDefinition { vector: vec, value: val, start_index: 0, end_index: vec.len() },
        IndexOfArrayInputs::Three((vec, val, start_index)) => IndexOfArrayDefinition { vector: vec, value: val, start_index: start_index, end_index: vec.len() },
        IndexOfArrayInputs::Four((vec, val, start_index, end_index)) => IndexOfArrayDefinition { vector: vec, value: val, start_index: start_index, end_index: end_index },
      }
    }
  }

  let indexOfArrayDef: IndexOfArrayDefinition = from_value(a)?;

  Ok(match indexOfArrayDef.vector[indexOfArrayDef.start_index..indexOfArrayDef.end_index]
      .iter()
      .position(|&item| item == indexOfArrayDef.value) {
        Some(index) => index.into(),
        None => (-1).into(),
    })
}

pub fn is_array(a: Value, genealogy: Vec<String>) -> Result<Value> {
  Ok(match a {
    Value::Array(..) => true.into(),
    _ => false.into(),
  })
}

pub fn map(a: Value, genealogy: Vec<String>) -> Result<Value> {

  #[derive(Serialize, Deserialize)]
  struct MapDefinition {
    input: Vec<Value>,
    r#in: Value,
    r#as: AsEnum,
  }

  let a_map: MapDefinition = from_value(a)?;

  a_map.input.iter().map(|&x| {
    let asKey: String = a_map.r#as.into();
    let mut context = Map::new();
    context.insert(asKey, x);
    aggregate(a_map.r#in, context.into(), genealogy)
  }).collect()
}

pub fn object_to_array(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_map: Map<String, Value> = from_value(a)?;

  #[derive(Serialize, Deserialize)]
  struct MappedKV {
    k: String,
    v: Value
  }
  
  let a_vec: Vec<Map<String, Value>> = a_map.iter().map(|(&key, &value)| {
    let kv_map = Map::new();
    kv_map.insert("k".into(), key.into());
    kv_map.insert("v".into(), value);
    kv_map
  }).collect();
  Ok(a_vec.into())
}

pub fn range(a: Value, genealogy: Vec<String>) -> Result<Value> {
  let a_vec = value_to_vec(a)?;
  let a_len = a_vec.len();
  match a_len {
    2 | 3 => {
      let start_value = f64::from(a_vec[0]);
      let end_value = number_to_f64(&value_to_number(&a_vec[1])?)?;
      let step_size = match a_len {
        3 => number_to_f64(&value_to_number(&a_vec[3])?)?,
        _ => -1 as f64,
      };
      let vec = (start_value..end_value)
        .step_by_float(step_size)
        .map(|x| f64_to_value(&x));

      let uvec = (0..2)
        .step_by(2)
        .map(|x| u64_to_value(&x));
      vec_to_value(&vec)
    }
    _ => Err("$range operator requires at least two inputs".to_string()),
  }
}



trait StepFloat: Sized {
  fn step_by_float(self, step: f64) -> StepByFloat;
}

impl StepFloat for std::ops::Range<f64> {
  fn step_by_float(self, step_size: f64) -> StepByFloat {
    StepByFloat {
      min: self.start,
      max: self.end,
      step: step_size,
      curr: self.start,
    }
  }
}

#[derive(Clone, Debug)]
pub struct StepByFloat {
    min: f64,
    max: f64,
    step: f64,
    curr: f64,
}

impl Iterator for StepByFloat {
  type Item = f64;
  fn next(&mut self) -> Option<f64> {
    if self.curr < self.max {
      self.curr = self.curr + self.step;
      Some(self.curr)
    } else {
      None
    }
  }
}
