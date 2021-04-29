use serde_json::{self, json, Value};
use std::{error::Error, fmt};

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
    match nested_item.as_array() {
        Some(item_array) => match item_array.last() {
            Some(unwrapped_value) => Ok(unwrapped_value.as_str().unwrap_or("").to_string()),
            None => Err(ParseError),
        },
        None => Err(ParseError),
    }
}

#[derive(Debug)]
pub struct ParseError;

impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing or flattening bulk array JSON text")
    }
}
