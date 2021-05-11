use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Unistring {
    ValidUtf8(String),
    InvalidUtf8 { invalid_utf8_string: Vec<u8> },
}

impl Unistring {
    pub fn get_string(&self) -> String {
        match self {
            Self::ValidUtf8(valid_utf8) => valid_utf8.to_string(),
            Self::InvalidUtf8 {
                invalid_utf8_string: invalid_bytes,
            } => get_invalid_utf8(invalid_bytes),
        }
    }
}

fn get_invalid_utf8(invalid_bytes: &[u8]) -> String {
    format!(
        r#"{{ "invalid_utf8_string": [{}] }}"#,
        invalid_bytes
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}
