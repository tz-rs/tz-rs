use super::ParseError;
use serde::de;
use serde_json::{self, json, Value};

#[derive(Debug)]
pub struct JsonArray<T> {
    flattened_vec: Vec<T>,
}

impl<T: de::DeserializeOwned> JsonArray<T> {
    pub fn from_str(str_to_parse: &str) -> Result<Self, ParseError> {
        let mut parse_response = serde_json::from_str::<Value>(str_to_parse)?;
        let parsed_json_array = parse_response
            .as_array_mut()
            .ok_or_else(|| generate_none_error("cannot parse initial json string as array"))?;

        let mut flattened_vec = Vec::new();
        for nested_entry in parsed_json_array {
            let parsed_item_value = match nested_entry.is_array() {
                true => unwrap_item_in_nested_json_array(nested_entry.take())?,
                false => nested_entry.take(),
            };
            let converted_item: T = serde_json::from_value(parsed_item_value)?;
            flattened_vec.push(converted_item);
        }

        Ok(Self { flattened_vec })
    }

    pub fn into_vec(self) -> Vec<T> {
        self.flattened_vec
    }
}

fn unwrap_item_in_nested_json_array(mut nested_item: Value) -> Result<Value, ParseError> {
    let nested_array = nested_item
        .as_array_mut()
        .ok_or_else(|| generate_none_error("invalid initial json array"))?;
    match nested_array.last() {
        Some(_) => Ok(nested_array.pop().unwrap()),
        None => Ok(json!("")),
    }
}

fn generate_none_error(detail: &str) -> ParseError {
    ParseError::ResponseParsingError(detail.to_string())
}
