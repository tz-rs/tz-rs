use super::ParseError;
use serde::de;
use serde_json::{self, json, Value};

pub struct JsonArray<T> {
    flattened_vec: Vec<T>,
}

impl<T: de::DeserializeOwned> JsonArray<T> {
    pub fn from_str(str_to_parse: &str) -> Result<Self, ParseError> {
        let parse_response = serde_json::from_str(str_to_parse).unwrap_or_else(|_| json!([]));
        let parsed_json_array = parse_response.as_array().unwrap();

        let mut flattened_vec = Vec::new();
        for nested_entry in parsed_json_array {
            let parsed_item_value = match nested_entry.is_array() {
                true => unwrap_item_in_nested_json_array(nested_entry)?,
                false => nested_entry,
            };
            let converted_item: T = convert_item_value_to_type(parsed_item_value)?;
            flattened_vec.push(converted_item);
        }

        Ok(Self { flattened_vec })
    }

    pub fn into_vec(self) -> Vec<T> {
        self.flattened_vec
    }
}

fn unwrap_item_in_nested_json_array(nested_item: &Value) -> Result<&Value, ParseError> {
    let generate_none_error = |detail: &str| ParseError::ResponseParsingError(detail.to_string());
    nested_item
        .as_array()
        .ok_or_else(|| generate_none_error("invalid initial json array"))?
        .last()
        .ok_or_else(|| generate_none_error("initial json array is empty"))
}

fn convert_item_value_to_type<T: de::DeserializeOwned>(
    item_value: &Value,
) -> Result<T, ParseError> {
    let item_json = json!(item_value);
    let converted_item: T = serde_json::from_value(item_json)?;
    Ok(converted_item)
}
