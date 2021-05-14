use super::ParseError;
use serde::{de, Deserialize, Serialize};
use serde_json::{self, Value};
use std::array::IntoIter;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonArray<T> {
    items: Vec<T>,
}

impl<T: de::DeserializeOwned + fmt::Debug> JsonArray<JsonArray<T>> {
    pub fn from_nested_response_str(nested_json_str: &str) -> Result<Self, ParseError> {
        let inner = Self::from_response_str(nested_json_str)?.into_vec();
        println!("{:?}", &inner);
        Ok(Self { items: inner })
    }
}

impl<T: de::DeserializeOwned + fmt::Debug> JsonArray<T> {
    pub fn from_response_str(str_to_parse: &str) -> Result<Self, ParseError> {
        let x = try_parse_array_into_item_from_response_str(str_to_parse);
        println!("{:?}", &x);
        // let items = try_parse_array_into_item_from_response_str(str_to_parse)?;
        let items = x?;
        Ok(Self { items })
    }

    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

fn try_parse_array_into_item_from_response_str<T: de::DeserializeOwned>(
    json_str: &str,
) -> Result<Vec<T>, ParseError> {
    let mut parse_response = serde_json::from_str::<Value>(json_str)?.take();
    let json_array = parse_response
        .as_array_mut()
        .ok_or_else(|| generate_none_error("cannot parse initial json string as array"))?;

    let mut item_vec = Vec::new();
    for json_item in json_array {
        let converted_item: [T; 1] = serde_json::from_value(json_item.take())?;
        for item in IntoIter::new(converted_item) {
            item_vec.push(item);
        }
    }

    Ok(item_vec)
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

fn generate_none_error(detail: &str) -> ParseError {
    ParseError::ResponseParsingError(detail.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_dimensional_json_array_from_str_parse_ok() {
        let mock_values = ["foo", "bar"];
        let mock_str_to_parse = format!(r#"[["{}"], ["{}"]]"#, mock_values[0], mock_values[1]);

        let parse_response = JsonArray::<String>::from_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let json_array = parse_response.unwrap();
        let string_to_compare = format!("[{}, {}]", mock_values[0], mock_values[1]);
        assert_eq!(json_array.to_string(), string_to_compare);

        assert_eq!(json_array.into_vec().len(), mock_values.len());
    }

    #[test]
    fn no_json_array_values_parse_ok() {
        let mock_str_to_parse = "[]";

        let parse_response = JsonArray::<String>::from_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let json_array = parse_response.unwrap().into_vec();
        assert_eq!(json_array.len(), 0);
    }

    #[test]
    fn empty_json_str_fail() {
        let mock_str_to_parse = "";

        let parse_response = JsonArray::<String>::from_response_str(&mock_str_to_parse);
        assert!(parse_response.is_err());
    }
}
