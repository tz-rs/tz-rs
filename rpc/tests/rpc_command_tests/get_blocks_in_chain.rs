use super::*;
use commands::GetBlocksInChain;

#[tokio::test]
async fn get_blocks_in_chain_ok() {
    let chain_id = get_chain_id_by_hash();
    let command = generate_get_blocks_command(chain_id);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block = client_response.unwrap();
    assert!(block.block_ids.len() > 0);
}

fn generate_get_blocks_command(chain_id: String) -> GetBlocksInChain {
    GetBlocksInChain::with_chain_id(chain_id)
}
