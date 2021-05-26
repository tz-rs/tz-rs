use super::*;
use chrono::{self, NaiveDateTime};
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
    let mut blocks = block_response.block_ids.into_vec();

    let last_block_batch = blocks.pop();
    assert!(last_block_batch.is_some());

    assert_eq!(last_block_batch.unwrap().len(), length as usize);
}

#[tokio::test]
async fn get_block_optional_min_date_at_epoch_ok() {
    let min_date = get_test_naive_datetime_at_epoch();
    let command = generate_get_blocks_command_with_explicit_params(None, None, Some(min_date));

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids;

    assert!(blocks.len() > 0);
}

#[tokio::test]
async fn get_block_optional_min_date_now_ok() {
    let min_date = get_naive_datetime_for_now();
    let command = generate_get_blocks_command_with_explicit_params(None, None, Some(min_date));

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids;

    assert_eq!(blocks.len(), 0);
}

#[tokio::test]
async fn get_blocks_with_min_date_and_length_ok() {
    let min_date = Some(get_test_naive_datetime_at_epoch());
    let length = Some(5);

    let command = generate_get_blocks_command_with_explicit_params(length, None, min_date);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids;

    assert_eq!(blocks.len(), length.unwrap() as usize);
}

#[tokio::test]
async fn get_blocks_with_head_and_length_ok() {
    let length = Some(5);
    let head_hash = Some("BKmXPAiniarmJAxvv3T3CQ9sa42TbLiDdE6c57Gc74NjcKNFQBE".to_string());

    let command = generate_get_blocks_command_with_explicit_params(length, head_hash, None);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids;

    assert_eq!(blocks.len(), 1);
}

#[tokio::test]
async fn get_blocks_all_optional_args_ok() {
    let length = Some(5);
    let min_date = Some(get_test_naive_datetime_at_epoch());
    let head_hash = Some("BKmXPAiniarmJAxvv3T3CQ9sa42TbLiDdE6c57Gc74NjcKNFQBE".to_string());

    let command = generate_get_blocks_command_with_explicit_params(length, head_hash, min_date);

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let block_response = client_response.unwrap();
    let blocks = block_response.block_ids;

    assert_eq!(blocks.len(), 1);
}

fn generate_get_blocks_command_with_explicit_params(
    length: Option<u32>,
    head_hash: Option<String>,
    min_date: Option<NaiveDateTime>,
) -> GetBlocksInChain {
    let chain_id = get_main_chain_id_by_tag();
    GetBlocksInChain::with_explicit_params(chain_id, length, head_hash, min_date)
}

fn get_naive_datetime_for_now() -> NaiveDateTime {
    chrono::offset::Utc::now().naive_utc()
}

fn get_test_naive_datetime_at_epoch() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}
