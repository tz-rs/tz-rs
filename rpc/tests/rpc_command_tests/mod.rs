use rpc::{commands, types, RpcClient};
use url::Url;

mod get_balance_from_block;
mod get_blocks_in_chain;

fn get_rpc_client() -> RpcClient {
    let tezos_node_url = get_local_testnet_url();
    RpcClient::new(tezos_node_url)
}

fn _get_public_testnet_url() -> Url {
    Url::parse("https://rpcalpha.tzbeta.net").unwrap()
}
fn get_local_testnet_url() -> Url {
    Url::parse("http://localhost:8090").unwrap()
}

fn get_main_chain_id_by_tag() -> types::Chain {
    types::Chain::Main
}

fn get_block_id_by_tag() -> types::Block {
    types::Block::Head
}
