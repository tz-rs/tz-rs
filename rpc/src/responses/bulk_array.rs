use serde_json::{self, json};

pub struct BulkArray<T> {
    flattened_vec: Vec<T>,
}

impl BulkArray<String> {
    pub fn from_str(str_to_parse: &str) -> Self {
        let parse_response = serde_json::from_str(str_to_parse).unwrap_or_else(|_| json!([]));
        let parsed_bulk_array = parse_response.as_array().unwrap();

        let mut flattened_vec = Vec::new();
        for nested_entry in parsed_bulk_array {
            let unwrapped_item = nested_entry
                .as_array()
                .unwrap()
                .last()
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            flattened_vec.push(unwrapped_item);
        }

        Self { flattened_vec }
    }

    pub fn into_vec(self) -> Vec<String> {
        self.flattened_vec
    }
}
