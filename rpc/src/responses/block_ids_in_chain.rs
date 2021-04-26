use super::Response;

pub struct BlocksInChainResponse {
    pub block_ids: Vec<String>,
}

impl Response for BlocksInChainResponse {
    /// Parses a response string in the form
    /// `"[["alpha_numeric_block_id_string"], ["..."]]"` into a
    /// [`BlocksInChainResponse`](Self).
    ///
    /// ## Note
    /// if the separator character/block ID delimiter changes in the
    /// future to something other than `,`, this method will fail.
    fn from_response_str(response: &str) -> Self {
        let mut block_id_vec = Vec::new();

        let mut current_block_id = String::new();
        let block_id_separator_char = ',';

        for raw_char in response.chars() {
            if raw_char == block_id_separator_char {
                let block_id_to_add = current_block_id.clone();
                block_id_vec.push(block_id_to_add);
                current_block_id.clear();
            }
            if raw_char.is_alphanumeric() {
                current_block_id.push(raw_char);
            }
        }

        // push last block ID if string is not empty
        if !current_block_id.is_empty() {
            block_id_vec.push(current_block_id);
        }

        Self {
            block_ids: block_id_vec,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_blocks_in_chain_from_response_empty_ok() {
        let mock_response = "";
        let blocks = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks.block_ids.len() == 0);
    }

    #[test]
    fn get_blocks_in_chain_from_response_single_ok() {
        let mock_block_id = "blockId1";
        let mock_response = format!(r#"[["{}"]]"#, mock_block_id);

        let mut blocks = BlocksInChainResponse::from_response_str(&mock_response);
        assert!(blocks.block_ids.len() == 1);

        let parsed_block_id_result = blocks.block_ids.pop();
        assert!(parsed_block_id_result.is_some());

        let parsed_block_id = parsed_block_id_result.unwrap();
        assert_eq!(&parsed_block_id, mock_block_id);
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

        let mut blocks = BlocksInChainResponse::from_response_str(&mock_response);
        assert!(blocks.block_ids.len() == 3);

        for mock_block_id in mock_block_ids.iter().rev() {
            let block_id_to_compare = blocks.block_ids.pop().unwrap();
            assert_eq!(&block_id_to_compare, mock_block_id);
        }
    }
}
