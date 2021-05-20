use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Unistring {
    ValidUtf8(String),
    InvalidUtf8 { invalid_utf8_string: Vec<u8> },
}

impl fmt::Display for Unistring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidUtf8(valid_utf8) => write!(f, "{}", valid_utf8.to_string()),
            Self::InvalidUtf8 {
                invalid_utf8_string: _,
            } => write!(f, "{}", json!(self).to_string()),
        }
    }
}
