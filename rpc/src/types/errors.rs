use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct ResponseError {
  title: String,
  description: String,
  r#type: String,
  properties: HashMap<String, Value>,
  required: Vec<String>,
  #[serde(rename(
    serialize = "additionalProperties",
    deserialize = "additionalProperties"
  ))]
  additional_properties: bool,
}

impl fmt::Display for ResponseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", json!(self).to_string())
  }
}
