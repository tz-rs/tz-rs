use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

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

impl ResponseError {
  pub fn get_string(&self) -> String {
    json!(self).to_string()
  }
}
