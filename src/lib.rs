#![recursion_limit = "1024"]
#[macro_use]
extern crate failure;
extern crate serde_json;
extern crate serde_derive;
extern crate bson;
extern crate serde;
extern crate regex;
extern crate log;
extern crate valico;
extern crate tantivy;


mod external;
mod utils;
mod operators;
mod error;

pub use external::query;
pub use external::aggregate;
pub use error::MingoError;
