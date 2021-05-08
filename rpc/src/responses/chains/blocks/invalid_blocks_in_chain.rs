use crate::errors::ParseError;
use crate::responses::{json_array, Response};
use crate::types::{ResponseError, Unistring};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InvalidBlock {
	block: Unistring,
	level: i32,
	errors: ResponseError,
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

	#[test]
	fn get_invalid_blocks_in_chain_from_response_empty_fails() {
		let mock_response = "";

		let blocks_response = InvalidBlocksInChainResponse::from_response_str(mock_response);
		assert!(blocks_response.is_err());
	}

	#[test]
	fn get_invalid_blocks_in_chain_from_response_ok() {
		let mock_block_id = "blockId1";
		let mock_block_level = 1;

		let mock_block_error = r#"{
			"title":"Identity keys mismatch",
			"description":"The current identity file has non-matching keys (secret key/ public key pair is not valid)",
			"type":"object",
			"properties":{
					"kind":{
							"type":"string",
							"enum":[
									"permanent"
							]
					}
			},
			"required":[
					"kind"
			],
			"additionalProperties":false
		}"#;
		let stripped_mock_block_error = mock_block_error.replace('\n', "").replace('\t', "");

		let mock_response = format!(
			r#"[{{"block":"{}", "level":{}, "errors":{}}}]"#,
			mock_block_id, mock_block_level, mock_block_error
		);

		let invalid_blocks_response = InvalidBlocksInChainResponse::from_response_str(&mock_response);
		assert!(invalid_blocks_response.is_ok());

		let mut invalid_blocks = invalid_blocks_response.unwrap().invalid_blocks.into_vec();
		assert!(invalid_blocks.len() == 1);

		let invalid_block = invalid_blocks.pop().unwrap();
		assert!(invalid_block.block.get_string() == mock_block_id);
		assert!(invalid_block.level == mock_block_level);
		assert_eq!(invalid_block.errors.get_string(), stripped_mock_block_error);
	}

	#[test]
	fn get_invalid_blocks_in_chain_from_malformed_response_fails() {
		let mock_block_id = "blockId1";
		let mock_block_level = 2;

		// Mock response without errors field
		let mock_response = format!(
			r#"[{{"block":"{}", "level":{}}}]"#,
			mock_block_id, mock_block_level
		);

		let invalid_blocks_response = InvalidBlocksInChainResponse::from_response_str(&mock_response);
		assert!(invalid_blocks_response.is_err());
	}
}
