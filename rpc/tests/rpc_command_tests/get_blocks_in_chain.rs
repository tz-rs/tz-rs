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
async fn get_blocks_with_optional_params_bad_head_hash_fail() {
    let bad_head_hash = "".to_string();
    let command = generate_get_blocks_command_with_explicit_params(None, Some(bad_head_hash), None);
    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_err());
}

#[tokio::test]
async fn get_blocks_from_head_optional_params_ok() {
    let head_hash = "BKmXPAiniarmJAxvv3T3CQ9sa42TbLiDdE6c57Gc74NjcKNFQBE".to_string();
    let command = generate_get_blocks_command_with_explicit_params(None, Some(head_hash), None);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids.into_vec();
    assert!(blocks.len() >= 1);
}

#[tokio::test]
async fn get_blocks_optional_length_ok() {
    let length = 5;
    let command = generate_get_blocks_command_with_explicit_params(Some(length), None, None);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids.into_vec();
    println!("{:?}", blocks);
    panic!();

    let last_block_batch = blocks.pop();
    assert!(last_block_batch.is_some());

    // assert_eq!(last_block_batch.unwrap().len(), length as usize);
}

fn generate_get_blocks_command_with_explicit_params(
    length: Option<u32>,
    head_hash: Option<String>,
    min_date: Option<NaiveDateTime>,
) -> GetBlocksInChain {
    let chain_id = get_main_chain_id_by_tag();
    GetBlocksInChain::with_explicit_params(chain_id, length, head_hash, min_date)
}

fn get_test_naive_datetime_at_epoc() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}
