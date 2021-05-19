use super::ParseError;
use serde::{de, Deserialize, Serialize};
use serde_json::{self, Value};
use std::array;
use std::fmt;
use std::iter;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonArray<T> {
    items: Vec<T>,
}

impl<T: de::DeserializeOwned> JsonArray<T> {
    pub fn from_response_str(str_to_parse: &str) -> Result<Self, ParseError> {
        let items = try_parse_array_into_item_from_response_str(str_to_parse)?;
        Ok(Self { items })
    }

    pub fn into_vec(self) -> Vec<T> {
        self.items
    }

    pub fn from_json_array(json_value: &mut Value) -> Result<Self, ParseError> {
        let array = json_value
            .as_array_mut()
            .ok_or_else(|| generate_none_error("cannot parse initial json string as array"))?;

        let mut items = Vec::new();
        for item in array {
            let converted_item = serde_json::from_value(item.take())?;
            items.push(converted_item);
        }
        Ok(Self { items })
    }

    pub fn from_json_object(json_value: &mut Value) -> Result<Self, ParseError> {
        // let object_map = json_value
        // .as_object_mut()
        // .ok_or_else(|| generate_none_error("cannot parse initial json string as object"))?;

        let converted_object = serde_json::from_value(json_value.take())?;
        let items = vec![converted_object];
        Ok(Self { items })
    }
}

impl<T: de::DeserializeOwned> JsonArray<JsonArray<T>> {
    pub fn from_nested_response_str(
        nested_json_str: &str,
    ) -> Result<JsonArray<JsonArray<T>>, ParseError> {
        let outer_array = try_json_str_into_json_array(nested_json_str)?;

        let mut items = Vec::new();
        for mut item in outer_array {
            let json_item = match &item {
                Value::Array(_) => JsonArray::from_json_array(&mut item),
                Value::Object(_) => JsonArray::from_json_object(&mut item),
                _ => {
                    let detail = "initial json value is neither an object or an array".to_string();
                    Err(ParseError::ResponseParsingError(detail))
                }
            }?;
            items.push(json_item);
        }
        Ok(JsonArray { items })
    }
}

impl<T> iter::IntoIterator for JsonArray<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: fmt::Display> fmt::Display for JsonArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.items.is_empty() {
            write!(f, "")
        } else {
            let mut display_string = String::new();
            for value in &self.items {
                display_string.push_str(&format!("{}, ", value));
            }
            display_string.truncate(display_string.len() - 2);
            write!(f, "{}", display_string)
        }
    }
}

fn try_parse_array_into_item_from_response_str<T: de::DeserializeOwned>(
    json_str: &str,
) -> Result<Vec<T>, ParseError> {
    let json_array = try_json_str_into_json_array(json_str)?;
    println!("ARR: {:?}", &json_array);
    let mut item_vec = Vec::new();
    for mut json_item in json_array {
        // let converted_item: [T; 1] = serde_json::from_value(json_item.take())?;
        let x = serde_json::from_value::<[T; 1]>(json_item.take());
        if let Err(e) = &x {
            println!("{:?}", &e);
        }
        let converted_item = x?;
        for item in array::IntoIter::new(converted_item) {
            item_vec.push(item);
        }
    }

    Ok(item_vec)
}

fn try_json_str_into_json_array(json_str: &str) -> Result<Vec<Value>, ParseError> {
    let mut parse_response = serde_json::from_str::<Value>(json_str)?.take();
    Ok(parse_response
        .as_array_mut()
        .ok_or_else(|| generate_none_error("cannot parse initial json string as array"))?
        .iter_mut()
        .map(|x| x.take())
        .collect())
}

fn generate_none_error(detail: &str) -> ParseError {
    ParseError::ResponseParsingError(detail.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::Unistring;

    // #[test]
    // fn parse_json_str_into_object_ok() {
    // struct MockObject {
    // foo: String,
    // bar: u32,
    // boolean: bool,
    // }

    // let mock_value = MockObject{foo: "foo".to_string(), bar: 10, boolean: false};
    // let mock_str_to_parse = format!("");

    // let parse_response = JsonArray::<MockObject>::from_response_str(&mock_str_to_parse);
    // assert!(parse_response.is_ok());

    // let json_array = parse_response.unwrap();

    // assert_eq!(
    // assert_eq!(json_array.into_vec().len(), mock_values.len());
    // }

    #[test]
    fn two_dimensional_string_json_array_from_nested_str_parse_ok() {
        let mock_values = [["foo", "bar"], ["foo_", "bar_"]];
        let mock_str_to_parse = format!("{:?}", &mock_values);

        type NestedType = JsonArray<String>;
        let parse_response = JsonArray::<NestedType>::from_nested_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let response = parse_response.unwrap();
        let tuple_vec = get_tuple_vec_from_response_and_mock_values(response, &mock_values);

        assert!(check_tuples_are_eq(tuple_vec));
    }

    #[test]
    fn two_dimensional_unistring_json_array_from_nested_str_parse_ok() {
        let mock_values = [["foo", "bar"], ["foo_", "bar_"]];
        let mock_str_to_parse = format!("{:?}", &mock_values);

        type NestedType = JsonArray<Unistring>;
        let parse_response = JsonArray::<NestedType>::from_nested_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let response = parse_response.unwrap();
        let tuple_vec = get_tuple_vec_from_response_and_mock_values(response, &mock_values);

        assert!(check_tuples_are_eq(tuple_vec));
    }

    #[test]
    fn one_dimensional_json_array_from_str_parse_ok() {
        let mock_values = ["foo", "bar"];
        let mock_str_to_parse = format!(r#"[["{}"], ["{}"]]"#, mock_values[0], mock_values[1]);

        let parse_response = JsonArray::<String>::from_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let json_array = parse_response.unwrap();
        let string_to_compare = format!("{}, {}", mock_values[0], mock_values[1]);
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

    fn get_tuple_vec_from_response_and_mock_values<'a, T, I: 'a>(
        response: JsonArray<JsonArray<T>>,
        mock_values: I,
    ) -> Vec<(T, <<I as IntoIterator>::Item as IntoIterator>::Item)>
    where
        T: de::DeserializeOwned,
        I: iter::IntoIterator,
        I::Item: iter::IntoIterator,
    {
        let mut zipped_tuple_iter = {
            let flat_response_iter = response.into_vec().into_iter().flatten();
            let mock_values_iter = mock_values.into_iter().flatten();

            flat_response_iter.into_iter().zip(mock_values_iter)
        };

        let mut arr = Vec::new();
        while let Some(tuple) = zipped_tuple_iter.next() {
            arr.push(tuple);
        }
        arr
    }

    fn check_tuples_are_eq<L, R>(tuple_vec: Vec<(L, &R)>) -> bool
    where
        L: PartialEq<R>,
    {
        for tuple in tuple_vec.iter() {
            if tuple.0 != *tuple.1 {
                return false;
            }
        }
        true
    }
}
