use crate::errors::ParseError;
use crate::responses::{json_array, Response};
use crate::types::Unistring;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct InvalidBlockError {
	pub kind: String,
	pub id: String,
	pub invalid_block: Unistring,
	pub error: String,
	#[serde(flatten)]
	pub extra_error_info: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct InvalidBlock {
	pub block: Unistring,
	pub level: i32,
	pub errors: InvalidBlockError,
}

pub struct InvalidBlocksInChainResponse {
	pub invalid_blocks: json_array::JsonArray<InvalidBlock>,
}

impl Response for InvalidBlocksInChainResponse {
	/// Parses a response string in the form
	/// `"[{ "block": "alpha_numeric_block_id_string",
	///      "level": integer ∈ [-2^31-1, 2^31],
	///      "errors": $error }, ...]"` into a
	/// [`InvalidBlocksInChainResponse`](Self).
	fn from_response_str(response: &str) -> Result<Self, ParseError> {
		let invalid_blocks = json_array::JsonArray::from_str(response)?;

		Ok(Self { invalid_blocks })
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use serde_json::json;

	fn get_mock_error_response(
		mock_kind: &str,
		mock_id: &str,
		mock_invalid_block_hash: &str,
		mock_error: &str,
		mock_extra_key: &str,
		mock_extra_value: &str,
	) -> String {
		let mock_error_response = format!(
			r#"{{
				"kind":"{}",
				"id":"{}",
				"invalid_block":"{}",
				"error":"{}",
				"{}":"{}"
			}}"#,
			mock_kind, mock_id, mock_invalid_block_hash, mock_error, mock_extra_key, mock_extra_value
		);

		mock_error_response.replace('\n', "").replace('\t', "")
	}

	fn get_mock_response(mock_block: &str, mock_level: i32, mock_error_response: &str) -> String {
		let mock_response = format!(
			r#"[{{
				"block":"{}",
				"level":{},
				"errors":{}
			}}]"#,
			mock_block, mock_level, mock_error_response
		);

		mock_response.replace('\n', "").replace('\t', "")
	}

	fn check_mock_error_json_matches_error_response(
		invalid_block_errors: InvalidBlockError,
		mock_error_response: &str,
	) -> bool {
		let invalid_block_errors_json = json!(invalid_block_errors).to_string();

		invalid_block_errors_json == mock_error_response
	}

	#[test]
	fn get_invalid_blocks_in_chain_from_response_empty_fails() {
		let mock_response = "";

		let blocks_response = InvalidBlocksInChainResponse::from_response_str(mock_response);
		assert!(blocks_response.is_err());
	}

	#[test]
	fn get_invalid_blocks_in_chain_from_response_ok() {
		let mock_block = "blockId1";
		let mock_level = 1;

		let mock_kind = "permanent";
		let mock_id = "validator.invalid_block";
		let mock_invalid_block_hash = "blockId1";
		let mock_error = "cannot_parse_operation";
		let mock_extra_key = "operation";
		let mock_extra_value = "operationHash1";

		let mock_error_response = get_mock_error_response(
			mock_kind,
			mock_id,
			mock_invalid_block_hash,
			mock_error,
			mock_extra_key,
			mock_extra_value,
		);

		let mock_response = get_mock_response(mock_block, mock_level, &mock_error_response);

		let invalid_blocks_response = InvalidBlocksInChainResponse::from_response_str(&mock_response);
		assert!(invalid_blocks_response.is_ok());

		let mut invalid_blocks = invalid_blocks_response.unwrap().invalid_blocks.into_vec();
		assert!(invalid_blocks.len() == 1);

		let invalid_block = invalid_blocks.pop().unwrap();
		assert!(invalid_block.block.to_string() == mock_block);
		assert!(invalid_block.level == mock_level);

		let invalid_block_errors = invalid_block.errors;
		let mock_error_matches_error_response =
			check_mock_error_json_matches_error_response(invalid_block_errors, &mock_error_response);
		assert!(mock_error_matches_error_response);
	}

	#[test]
	fn get_invalid_blocks_in_chain_from_malformed_response_fails() {
		let mock_block = "blockId2";
		let mock_level = 2;

		// Mock response without errors field, which is required for all type of errors
		let mock_response = format!(
			r#"[{{
				"block":"{}",
				"level":{},
			}}]"#,
			mock_block, mock_level
		);

		let invalid_blocks_response = InvalidBlocksInChainResponse::from_response_str(&mock_response);
		assert!(invalid_blocks_response.is_err());
	}
}
