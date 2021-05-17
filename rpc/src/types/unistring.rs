use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Unistring {
    ValidUtf8(String),
    InvalidUtf8 { invalid_utf8_string: Vec<u8> },
}

impl PartialEq<str> for &Unistring {
    fn eq(&self, other: &str) -> bool {
        match self {
            Unistring::ValidUtf8(utf8_string) => utf8_string == other,
            _ => false,
        }
    }
}

impl PartialEq<&str> for Unistring {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::ValidUtf8(utf8_string) => utf8_string == other,
            _ => false,
        }
    }
}

impl PartialEq<&&str> for Unistring {
    fn eq(&self, other: &&&str) -> bool {
        match self {
            Self::ValidUtf8(utf8_string) => utf8_string == *other,
            _ => false,
        }
    }
}

impl PartialEq<str> for Unistring {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::ValidUtf8(utf8_string) => utf8_string == other,
            _ => false,
        }
    }
}

impl fmt::Display for Unistring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidUtf8(valid_utf8) => write!(f, "{}", valid_utf8),
            Self::InvalidUtf8 {
                invalid_utf8_string: _,
            } => write!(f, "{}", json!(self)),
        }
    }
}
