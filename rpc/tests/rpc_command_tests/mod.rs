use rpc::rpc_commands;
use rpc::RPCClient;
use url::Url;

mod get_balance_from_block;
mod get_blocks_in_chain;

fn get_chain_id_string() -> String {
    // NOTE: gets the ID string of the chain avaible on florencenet.
    // might change sometime.
    "NetXdQprcVkpaWU".to_string()
}

fn get_rpc_client() -> RPCClient {
    let tezos_node_url = get_local_testnet_url();
    RPCClient::new(tezos_node_url)
}

fn _get_public_testnet_url() -> Url {
    Url::parse("https://rpcalpha.tzbeta.net").unwrap()
}
fn get_local_testnet_url() -> Url {
    Url::parse("http://localhost:8090").unwrap()
}

fn get_chain_id_for_public_testnet() -> String {
    // IMPORTANT: This is specifically for https://rpcalpha.tzbeta.net testnet
    "main".to_string()
}

fn get_block_id_for_public_testnet() -> String {
    // IMPORTANT: This is specifically for https://rpcalpha.tzbeta.net testnet
    "head".to_string()
}
