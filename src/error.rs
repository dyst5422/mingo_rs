use serde_json;

error_chain! {
  errors {
    MingoError(genealogy: Vec<String>) {
      description("Mingo Processing Error")
      display("Path to error: '{}'", genealogy.join("."))
    }
  }
  foreign_links {
    Serde(serde_json::Error);
    Regex(regex::Error);
  }
}
