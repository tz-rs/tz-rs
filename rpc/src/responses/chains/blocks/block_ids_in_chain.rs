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
    fn get_blocks_in_chain_from_response_empty_fail() {
        let mock_response = "";

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_err());
    }

    #[test]
    fn get_blocks_in_chain_from_empty_list_ok() {
        let mock_response = "[]";
        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_ok());

        let blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.is_empty());
    }

    #[test]
    fn get_blocks_in_chain_from_empty_bulk_array_response_ok() {
        let mock_response = "[[]]";

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_ok());

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        let parsed_block = blocks.pop().unwrap();
        assert!(parsed_block.to_string() == "");
    }

    #[test]
    fn get_blocks_in_chain_from_response_single_ok() {
        let mock_block_id = "blockId1";
        let mock_response = format!(r#"[["{}"]]"#, mock_block_id);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response);
        println!("{:?}", blocks_response);
        panic!();
        assert!(blocks_response.is_ok());

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
        let mock_response = format!(r#"[[{}]]"#, mock_nested_object);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response);
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
        let mock_response = format!(
            "[{}]",
            mock_block_ids
                .iter()
                .map(|block_id| format!(r#"["{}"]"#, &block_id))
                .collect::<Vec<String>>()
                .join(",")
        );

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response);
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
