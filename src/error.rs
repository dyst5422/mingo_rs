use failure::Fail;
use serde_json::Value;

#[derive(Debug, Fail)]
pub enum MingoError {
  #[fail(display = "Invalid Argument {} at {}\n{}", argument, genealogy, message)]
  ArgumentError {
    message: &'static str,
    argument: Value,
    genealogy: String,
  },
  #[fail(display = "Unimplemented operation {} at {}\n{}", operation, genealogy, message)]
  UnimplementedOperationError {
    message: &'static str,
    operation: String,
    genealogy: String,
  },
  #[fail(display = "Unrecognized query operation {} at {}", operation, genealogy)]
  UnknownQueryOperationError {
    // message: &'static str,
    operation: String,
    genealogy: String,
  },
  #[fail(display = "Unrecognized aggregation operation {} at {}", operation, genealogy)]
  UnknownAggregationOperationError {
    // message: &'static str,
    operation: String,
    genealogy: String,
  },
  #[fail(display = "Unknown type operation value {} at {}\n{}", argument, genealogy, message)]
  UnknownTypeOperationValueError {
    message: &'static str,
    argument: Value,
    genealogy: String,
  },
  #[fail(display = "Improperly formatted query at {}\n{}", genealogy, message)]
  ImproperlyFormattedQueryError {
    message: &'static str,
    genealogy: String,
  },
  #[fail(display = "Improperly formatted aggregation at {}\n{}", genealogy, message)]
  ImproperlyFormattedAggregationError {
    message: &'static str,
    genealogy: String,
  }
}

pub type Result<T> = std::result::Result<T, failure::Error>;
