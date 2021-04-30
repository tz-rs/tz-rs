use rpc::{commands, types, RpcClient};
use std::env;
use url::Url;

mod get_balance_from_block;
mod get_blocks_in_chain;

fn get_rpc_client() -> RpcClient {
    let tezos_node_url = match is_testing_on_cloud() {
        true => get_local_testnet_url(),
        false => get_public_testnet_url(),
    };
    RpcClient::new(tezos_node_url)
}

fn get_public_testnet_url() -> Url {
    Url::parse("https://tezos-florence.cryptonomic-infra.tech:443").unwrap()
}
fn get_local_testnet_url() -> Url {
    Url::parse("http://localhost:8090").unwrap()
}

fn is_testing_on_cloud() -> bool {
    let is_deployed_env_key = "CI";
    match env::var(is_deployed_env_key) {
        Ok(val) => val == "true",
        Err(_) => false,
    }
}

fn get_main_chain_id_by_tag() -> types::Chain {
    types::Chain::Main
}

fn get_block_id_by_tag() -> types::Block {
    types::Block::Head
}
