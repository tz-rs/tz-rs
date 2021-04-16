use super::*;
use commands::GetBlocksInChain;
use responses::BlocksInChainResponse;
use std::str::FromStr;

#[tokio::test]
async fn get_blocks_in_chain_ok() {
    let chain_id = get_chain_id_string();
    let command = generate_boxed_get_blocks_command(chain_id);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let raw_response = client.execute(&command).await;
    assert!(raw_response.is_ok());

    let response = raw_response.unwrap();
    assert_eq!(response.status(), 200);

    let response_text = response.text().await.unwrap();

    let block = BlocksInChainResponse::from_str(&response_text).unwrap();

    assert!(block.block_ids.len() > 0);
}

#[tokio::test]
async fn blocks_in_chain_response_parseable() {
    let chain_id = get_chain_id_string();
    let response_text = get_response_from_get_blocks_in_chain_call(chain_id)
        .await
        .unwrap();

    let parse_result = BlocksInChainResponse::from_str(&response_text);
    assert!(parse_result.is_ok());

    let blocks = parse_result.unwrap();
    assert!(blocks.block_ids.len() > 0);
}

async fn get_response_from_get_blocks_in_chain_call(chain_id: String) -> std::io::Result<String> {
    let command = generate_boxed_get_blocks_command(chain_id);
    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let raw_response = client.execute(&command).await;
    assert!(raw_response.is_ok());

    let response = raw_response.unwrap();
    assert_eq!(response.status(), 200);

    Ok(response.text().await.unwrap())
}

fn generate_boxed_get_blocks_command(chain_id: String) -> Box<GetBlocksInChain> {
    Box::new(GetBlocksInChain { chain_id })
}
