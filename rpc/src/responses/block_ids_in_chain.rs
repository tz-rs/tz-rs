use std::str::FromStr;
use std::string::ParseError;

pub struct BlocksInChainResponse {
    pub block_ids: Vec<String>,
}

impl FromStr for BlocksInChainResponse {
    type Err = ParseError;

    /// Parses a response string in the form
    /// `"[["alpha_numeric_block_id_string"], ["..."]]"` into a
    /// [`BlocksInChainResponse`](Self).
    ///
    /// ## Note
    /// if the separator character/block ID delimiter changes in the
    /// future to something other than `,`, this method will fail.
    fn from_str(response_text: &str) -> Result<Self, Self::Err> {
        let mut block_id_vec = Vec::new();

        let mut current_block_id = String::new();
        let block_id_separator_char = ',';

        for raw_char in response_text.chars() {
            if raw_char == block_id_separator_char {
                let block_id_to_add = current_block_id.clone();
                block_id_vec.push(block_id_to_add);
                current_block_id.clear();
            }
            if raw_char.is_alphanumeric() {
                current_block_id.push(raw_char);
            }
        }
        // push last block ID
        block_id_vec.push(current_block_id);

        Ok(Self {
            block_ids: block_id_vec,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_blocks_in_chain_response_from_str() {
        let mock_response = "";
        let parse_result = BlocksInChainResponse::from_str(mock_response);
        assert!(parse_result.is_ok());

        let blocks = parse_result.unwrap();
        assert!(blocks.block_ids.len() > 0);
    }
}
