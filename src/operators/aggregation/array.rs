use error::*;
use external::aggregate;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Value};
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

pub fn filter(a: &Value, _context: &Value, genealogy: &Vec<String>) -> Result<Value> {
  #[derive(Serialize, Deserialize)]
  struct FilterDefinition {
    input: Vec<Value>,
    cond: Value,
    r#as: Option<String>,
  }
  let FilterDefinition { input: filter_input, cond: filter_cond, r#as: filter_as_option } = from_value(a)?;
  let filter_as = match filter_as_option {
    Some(this_key) => this_key,
    None => "this".to_string(),
  };

  let filtered_iterator = filter_input
    .iter()
    .filter_map(|x| {
      let mut context = Map::new();
      context.insert(filter_as.clone(), x.clone());

      let cond_result = aggregate(&filter_cond, &context.into(), genealogy);
      match cond_result {
        Ok(val) => match val.as_bool() {
          Some(val_bool) => match val_bool {
            true => Some(x.clone()),
            false => None,
          },
          None => None
        },
        Err(_) => None
      }
    });

    let filtered_collection: Vec<Value> = filtered_iterator.collect();
    Ok(filtered_collection.into())
}

pub fn in_op(a: &Value, _context: &Value, _genealogy: &Vec<String>) -> Result<Value> {
  let (a_val, b_vec): (Value, Vec<Value>) = from_value(a)?;
  Ok(b_vec.contains(&a_val).into())
}

pub fn index_of_array(a: &Value, _context: &Value, genealogy: &Vec<String>) -> Result<Value> {

  #[derive(Serialize, Deserialize, Debug)]
  struct IndexOfArrayDefinition {
    vector: Vec<Value>,
    value: Value,
    start_index: usize,
    end_index: usize,
  }

  let index_of_array_inputs: Vec<Value> = from_value(a)?;

  let index_of_array_def = match index_of_array_inputs.len() {
    2 => {
      let vector: Vec<Value> = from_value(&index_of_array_inputs[0])?;
      Ok(IndexOfArrayDefinition {
        end_index: vector.len(),
        vector,
        value: index_of_array_inputs[1].clone(),
        start_index: 0,
      })
    },
    3 => {
      let vector: Vec<Value> = from_value(&index_of_array_inputs[0])?;
      let start_index: usize = from_value(&index_of_array_inputs[2])?;
      Ok(IndexOfArrayDefinition {
        end_index: vector.len(),
        vector,
        value: index_of_array_inputs[1].clone(),
        start_index,
      })
    },
    4 => {
      let vector: Vec<Value> = from_value(&index_of_array_inputs[0])?;
      let start_index: usize = from_value(&index_of_array_inputs[2])?;
      let end_index: usize = from_value(&index_of_array_inputs[3])?;
      Ok(IndexOfArrayDefinition {
        end_index,
        vector,
        value: index_of_array_inputs[1].clone(),
        start_index,
      })
    },
    _ => Err(MingoError::ImproperlyFormattedAggregationError { genealogy: genealogy.join(","), message: ""})
  }?;

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
    r#as: Option<String>,
  }

  let MapDefinition { input: map_input, r#in: map_in, r#as: map_as_option } = from_value(a)?;
  let map_as = match map_as_option {
    Some(this_key) => this_key,
    None => "this".to_string(),
  };

  map_input
    .iter()
    .map(|x| {
      let mut context = Map::new();
      context.insert(map_as.clone(), x.clone());
      aggregate(&map_in, &context.into(), genealogy)
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
