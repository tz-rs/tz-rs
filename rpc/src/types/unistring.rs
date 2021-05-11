use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Unistring {
  ValidUtf8(String),
  InvalidUtf8 { invalid_utf8_string: Vec<u8> },
}

impl Unistring {
  pub fn to_string(&self) -> String {
    match self {
      Self::ValidUtf8(valid_utf8) => valid_utf8.to_string(),
      Self::InvalidUtf8 {
        invalid_utf8_string: _,
      } => json!(self).to_string(),
    }
  }
}
