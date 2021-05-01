use super::ParseError;
use serde_json::{self, json, Value};

pub struct BulkArray<T> {
    flattened_vec: Vec<T>,
}

impl BulkArray<String> {
    pub fn from_str(str_to_parse: &str) -> Result<Self, ParseError> {
        let parse_response = serde_json::from_str(str_to_parse).unwrap_or_else(|_| json!([]));
        let parsed_bulk_array = parse_response.as_array().unwrap();

        let mut flattened_vec = Vec::new();
        for nested_entry in parsed_bulk_array {
            let parsed_item_string = unwrap_string_in_nested_json_array(nested_entry)?;
            flattened_vec.push(parsed_item_string);
        }

        Ok(Self { flattened_vec })
    }

    pub fn into_vec(self) -> Vec<String> {
        self.flattened_vec
    }
}

fn unwrap_string_in_nested_json_array(nested_item: &Value) -> Result<String, ParseError> {
    let generate_none_error = |detail: &str| ParseError::ResponseParsingError(detail.to_string());
    Ok(nested_item
        .as_array()
        .ok_or_else(|| generate_none_error("invalid initial json array"))?
        .last()
        .ok_or_else(|| generate_none_error("initial json array is empty"))?
        .as_str()
        .ok_or_else(|| generate_none_error("json array value is not a string"))?
        .to_string())
}
