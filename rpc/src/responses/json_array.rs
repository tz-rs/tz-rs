use super::ParseError;
use serde::{de, Deserialize, Serialize};
use serde_json::{self, Value};
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
        let converted_object = serde_json::from_value(json_value.take())?;
        let items = vec![converted_object];
        Ok(Self { items })
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
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

    pub fn into_flattened_vec(self) -> Vec<T> {
        self.into_iter().flatten().collect()
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
    let mut item_vec = Vec::new();

    for mut json_item in json_array {
        let converted_item = serde_json::from_value(json_item.take())?;
        item_vec.push(converted_item);
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

    #[test]
    fn parse_json_str_into_object_ok() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        struct MockObject {
            foo: String,
            bar: u32,
            boolean: bool,
        }

        let mock_object_1 = MockObject {
            foo: "foo".to_string(),
            bar: 10,
            boolean: false,
        };

        let mock_object_2 = MockObject {
            foo: "foo_".to_string(),
            bar: 100,
            boolean: true,
        };

        fn mock_object_to_str(obj: &MockObject) -> String {
            let data = format!(
                r#""foo": "{}", "bar": {}, "boolean": {}"#,
                obj.foo, obj.bar, obj.boolean
            );
            ["{", &data, "}"].join("")
        }

        let mock_str_to_parse = format!(
            "[{}, {}]",
            mock_object_to_str(&mock_object_1),
            mock_object_to_str(&mock_object_2),
        );

        let parse_response = JsonArray::<MockObject>::from_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let json_array = parse_response.unwrap();

        let mock_value_arr = [mock_object_1, mock_object_2];
        assert_eq!(json_array.len(), mock_value_arr.len());

        let mock_value_and_parsed_value_tuple_iter =
            mock_value_arr.iter().zip(json_array.into_iter());

        for tuple in mock_value_and_parsed_value_tuple_iter {
            let mock_val = tuple.0;
            let parsed_val = &tuple.1;
            assert_eq!(mock_val, parsed_val);
        }
    }

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
    fn two_dimensional_string_json_array_parsed_and_flattened_ok() {
        let mock_values = [["foo", "bar"], ["foo_", "bar_"]];
        let mock_str_to_parse = format!("{:?}", &mock_values);

        type NestedType = JsonArray<String>;
        let parse_response = JsonArray::<NestedType>::from_nested_response_str(&mock_str_to_parse);
        assert!(parse_response.is_ok());

        let response = parse_response.unwrap();

        let flattened_mock_value_vec: Vec<&&str> = mock_values.iter().flatten().collect();
        let flattened_response_vec = response.into_flattened_vec();

        assert_eq!(flattened_response_vec.len(), flattened_mock_value_vec.len());

        let mut zipped_iters = flattened_mock_value_vec
            .into_iter()
            .zip(flattened_response_vec);

        while let Some(tuple) = zipped_iters.next() {
            let response = tuple.1;
            let mock = *tuple.0;
            assert_eq!(response, mock);
        }
    }

    #[test]
    fn one_dimensional_json_array_of_strings_from_str_parse_ok() {
        let mock_values = ["foo", "bar"];
        let mock_str_to_parse = format!("{:?}", &mock_values);

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
