use super::*;
use chrono::NaiveDateTime;
use commands::chains::blocks::get_blocks_in_chain::GetBlocksInChain;

#[tokio::test]
async fn get_blocks_in_chain_ok() {
    let command = generate_get_blocks_command_for_main_chain();

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids.into_vec();
    assert!(blocks.len() > 0);
}

fn generate_get_blocks_command_for_main_chain() -> GetBlocksInChain {
    let chain_id = get_main_chain_id_by_tag();
    GetBlocksInChain::with_default_params(chain_id)
}

#[tokio::test]
async fn get_blocks_in_chain_optional_params_ok() {
    let command = generate_get_blocks_command_with_explicit_params();

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    println!("resp: {:?}", block_response);
    panic!();
    let blocks = block_response.block_ids.into_vec();
    assert!(blocks.len() > 0);
}

fn generate_get_blocks_command_with_explicit_params() -> GetBlocksInChain {
    let chain_id = get_main_chain_id_by_tag();
    let length = Some(10);
    let head = Some(String::from(""));
    let min_date = Some(get_test_naive_datetime_at_epoc());
    GetBlocksInChain::with_explicit_params(chain_id, length, head, min_date)
}

fn get_test_naive_datetime_at_epoc() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}
