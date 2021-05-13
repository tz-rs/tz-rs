use super::ParseError;
use serde::{de, Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonArray<T> {
    items: Vec<T>,
}

impl<T: de::DeserializeOwned> JsonArray<T> {
    pub fn from_response_str(str_to_parse: &str) -> Result<Self, ParseError> {
        let parsed_array = try_parse_array_from_json_str(str_to_parse)?;

        let mut items = Vec::new();

        for item in parsed_array {
            let converted_item: T = serde_json::from_value(item);
            items.push(converted_item);
        }

        Ok(Self { items })
    }

    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

fn try_parse_array_from_json_str<T: de::DeserializeOwned>(
    json_str: &str,
) -> Result<Vec<T>, ParseError> {
    let mut parse_response = serde_json::from_str::<Value>(json_str)?;
    parse_response
        .as_array_mut()
        .ok_or_else(|| generate_none_error("cannot parse initial json string as array"))
        .take()
}

impl<T: fmt::Display> fmt::Display for JsonArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::from("[");
        for value in &self.items {
            display_string.push_str(&format!("{}, ", value.to_string()));
        }
        display_string.truncate(display_string.len() - 2);
        display_string.push_str("]");
        write!(f, "{}", display_string)
    }
}

fn unwrap_item_in_nested_json_array<T: de::DeserializeOwned>(
    mut nested_item: Value,
) -> Result<T, ParseError> {
    let nested_array = nested_item
        .as_array_mut()
        .ok_or_else(|| generate_none_error("invalid initial json array"))?;

    for item in nested_array {
        let converted_item: T = serde_json::from_value(item);
    }

    // match nested_array.last() {
    // Some(_) => Ok(nested_array.pop().unwrap()),
    // None => Ok(json!("")),
    // }
}

fn generate_none_error(detail: &str) -> ParseError {
    ParseError::ResponseParsingError(detail.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_dimensional_json_array_parse_ok() {
        let mock_values = ["foo", "bar"];
        let mock_str_to_parse = format!(r#"[["{}"], ["{}"]]"#, mock_values[0], mock_values[1]);

        let parse_response = JsonArray::<String>::from_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let json_array = parse_response.unwrap();
        let string_to_compare = format!("[{}, {}]", mock_values[0], mock_values[1]);
        assert_eq!(json_array.to_string(), string_to_compare);

        assert_eq!(json_array.into_vec().len(), mock_values.len());
    }

    #[test]
    fn no_json_array_values_parse_ok() {
        let mock_str_to_parse = "[]";

        let parse_response = JsonArray::<String>::from_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let json_array = parse_response.unwrap().into_vec();
        assert_eq!(json_array.len(), 0);
    }

    #[test]
    fn empty_json_str_fail() {
        let mock_str_to_parse = "";

        let parse_response = JsonArray::<String>::from_str(&mock_str_to_parse);
        assert!(parse_response.is_err());
    }
}
