use super::*;
use rpc_commands::GetBlocksInChain;

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
}

fn generate_boxed_get_blocks_command(chain_id: String) -> Box<GetBlocksInChain> {
    Box::new(GetBlocksInChain { chain_id })
}
