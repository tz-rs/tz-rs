use super::*;
use commands::GetBlocksInChain;

#[tokio::test]
async fn get_blocks_in_chain_ok() {
    let command = generate_get_blocks_command_for_main_chain();

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids.to_vec();
    assert!(blocks.len() > 0);
}

fn generate_get_blocks_command_for_main_chain() -> GetBlocksInChain {
    let chain_id = get_main_chain_id_by_tag();
    GetBlocksInChain { chain_id }
}
