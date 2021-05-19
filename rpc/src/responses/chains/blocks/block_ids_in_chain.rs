use crate::errors::ParseError;
use crate::responses::{json_array::JsonArray, Response};
use crate::types::Unistring;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlocksInChainResponse {
    pub block_ids: JsonArray<JsonArray<Unistring>>,
}

impl fmt::Display for BlocksInChainResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", json!(self).to_string())
    }
}

impl Response for BlocksInChainResponse {
    /// Parses a response string in the form
    /// `"[["alpha_numeric_block_id_string", "..."], ["...", "..."]]"` or
    /// `[[{ "invalid_utf8_string": [ integer ∈ [0, 255] ... ] }], [...]]` into a
    /// [`BlocksInChainResponse`](Self).
    fn from_response_str(response: &str) -> Result<Self, ParseError> {
        let block_ids = JsonArray::from_nested_response_str(response)?;

        Ok(Self { block_ids })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_blocks_in_chain_from_nested_irregular_array_ok() {
        let mock_arr_1 = ["blockId1", "blockId2", "blockId3"];
        let mock_arr_2 = ["blockId4", "blockId5"];
        let mock_response_str = format!("[{:?}, {:?}]", &mock_arr_1, &mock_arr_2);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response_str);
        assert!(blocks_response.is_ok());

        let blocks = blocks_response.unwrap().block_ids.into_vec();
        let mut zipped_tuple_iter = mock_arr_1
            .iter()
            .chain(mock_arr_2.iter())
            .zip(blocks.into_iter().flatten());

        while let Some(tuple) = zipped_tuple_iter.next() {
            let mock_str = *tuple.0;
            let parsed_response = tuple.1;
            assert_eq!(parsed_response, mock_str);
        }
    }

    #[test]
    fn get_blocks_in_chain_from_nested_regular_array_ok() {
        let mock_block_id = [
            ["blockId1", "blockId2", "blockId3"],
            ["blockId4", "blockId5", "blockId6"],
        ];
        let mock_response_str = format!("{:?}", &mock_block_id);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response_str);

        let blocks = blocks_response.unwrap().block_ids.into_vec();
        assert_eq!(blocks.len(), mock_block_id.len());

        let mut zipped_tuple_iter = mock_block_id
            .iter()
            .flatten()
            .zip(blocks.into_iter().flatten());

        while let Some(tuple) = zipped_tuple_iter.next() {
            let mock_str = *tuple.0;
            let parsed_response = tuple.1;
            assert_eq!(parsed_response, mock_str);
        }
    }

    #[test]
    fn get_blocks_in_chain_from_response_empty_fail() {
        let mock_response_str = "";

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response_str);
        assert!(blocks_response.is_err());
    }

    #[test]
    fn get_blocks_in_chain_from_empty_list_ok() {
        let mock_response_str = "[]";
        let blocks_response = BlocksInChainResponse::from_response_str(mock_response_str);
        assert!(blocks_response.is_ok());

        let blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.is_empty());
    }

    #[test]
    fn get_blocks_in_chain_from_empty_bulk_array_response_ok() {
        let mock_response_str = "[[]]";

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response_str);
        assert!(blocks_response.is_ok());

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        let parsed_block = blocks.pop().unwrap();
        assert!(parsed_block.to_string() == "");
    }

    #[test]
    fn get_blocks_in_chain_from_response_single_ok() {
        let mock_block_id = "blockId1";
        let mock_response_str = format!(r#"[["{}"]]"#, mock_block_id);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response_str);

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.len() == 1);

        let parsed_block = blocks.pop();
        assert!(parsed_block.is_some());

        let parsed_block_id = parsed_block.unwrap().to_string();
        assert_eq!(parsed_block_id, mock_block_id);
    }

    #[test]
    fn get_blocks_in_chain_from_invalid_utf8_response_single_ok() {
        let mock_nested_object = r#"{"invalid_utf8_string":[1,2,3,4]}"#;
        let mock_response_str = format!(r#"[[{}]]"#, mock_nested_object);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response_str);
        assert!(blocks_response.is_ok());

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.len() == 1);

        let parsed_block = blocks.pop();
        assert!(parsed_block.is_some());

        let parsed_block_id = parsed_block.unwrap().to_string();
        assert_eq!(parsed_block_id, mock_nested_object);
    }

    #[test]
    fn get_blocks_in_chain_from_response_multiple_ok() {
        let mock_block_ids = ["blockId1", "blockId2", "blockId3"];
        let mock_response_str = format!(
            "[{}]",
            mock_block_ids
                .iter()
                .map(|block_id| format!(r#"["{}"]"#, &block_id))
                .collect::<Vec<String>>()
                .join(",")
        );

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response_str);
        assert!(blocks_response.is_ok());

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.len() == 3);

        for mock_block_id in mock_block_ids.iter().rev() {
            let parsed_block = blocks.pop().unwrap();
            let parsed_block_id = parsed_block.to_string();
            assert_eq!(parsed_block_id, mock_block_id.to_string());
        }
    }
}
