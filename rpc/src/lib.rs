//! # tz-rpc
//!
//! tz-rpc serves as the top-level wrapper around raw JSON-RPC calls
//! to a Tezos net service/implementor
//!
//! ## Features
//!
//! - Asynchronous design
//! - Familiar and idiomatic Rust SDK
//! - Trait-driven extensible design for RPC commands
//! - Highly configurable for different Tezos use cases

pub mod rpc_commands;
use reqwest;
use rpc_commands::RPCClientCommand;
use url::Url;

/// Client wrapper and executor for making RPC calls to the Tezos net.
///
/// Execute commands implementing [`RPCClientCommand`] by passing them
/// into [`execute()`](Self::execute())
///
/// Should be instanciated only once and re-used so as to not reinstanciate
/// the inner `reqwest` client
pub struct RPCClient {
    tezos_node_url: Url,
    client: reqwest::Client,
}

impl RPCClient {
    /// Instanciates a re-usable client with the main resolving endpoint
    /// set to the URL passed in.
    ///
    /// This should be either a `localhost` address with port included, or
    /// the address of a public mainnet or testnet node.
    pub fn new(tezos_node_url: Url) -> Self {
        let client = reqwest::Client::new();
        Self {
            tezos_node_url,
            client,
        }
    }

    /// Sends a `GET` request to the node's `/version` endpoint,
    /// checking that the response was Ok and returned a status
    /// code in the 200s.
    ///
    /// Can be used as a pre-flight `OPTIONS` check to fail fast on node connections
    pub async fn check_node_online(&self) -> bool {
        let default_url = format!("{}version", &self.tezos_node_url);
        let response_result = self.client.get(default_url).send().await;

        match response_result {
            Ok(response) => response.status() == 200,
            Err(_) => false,
        }
    }

    /// Makes the JSON RPC request to the endpoint specified by the
    /// [`command`](RPCClientCommand) passed in.
    ///
    /// Returns the unparsed response so as to allow for explicit error checking
    /// directly with the [`reqwest::Response`] object.
    pub async fn execute<T: RPCClientCommand>(
        &self,
        command: &Box<T>,
    ) -> reqwest::Result<reqwest::Response> {
        let raw_endpoint_url = format!("{}{}", self.tezos_node_url, command.get_url_string());
        let endpoint_url = reqwest::Url::parse(&raw_endpoint_url).unwrap();

        let request = self.client.request(command.get_http_method(), endpoint_url);
        let response = request.send().await;

        response
    }
}

#[cfg(test)]
mod test_rpc_client {
    use super::rpc_commands::{GetBalance, GetBlocksInChain};
    use super::*;

    #[test]
    fn rpc_client_creation_ok() {
        let tezos_node_url = Url::parse("http://localhost").unwrap();
        RPCClient::new(tezos_node_url);
    }

    #[tokio::test]
    async fn invalid_url_fails_health_check() {
        let invalid_url = "http://localhost:8091";
        let tezos_node_url = Url::parse(invalid_url).unwrap();
        let client = RPCClient::new(tezos_node_url);

        let health_check_invalid = !client.check_node_online().await;
        assert!(health_check_invalid);
    }

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

    #[tokio::test]
    async fn get_balance_for_bob_ok() {
        let chain_id = get_chain_id_for_public_testnet();
        let block_id = get_block_id_for_public_testnet();
        let address = get_address_for_fake_wallet_bob();

        let command = generate_boxed_get_balance_command(chain_id, block_id, address);
        let client = get_rpc_client();
        assert!(client.check_node_online().await);

        let raw_response = client.execute(&command).await;
        assert!(raw_response.is_ok());

        let response = raw_response.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn get_balance_for_alice_ok() {
        let chain_id = get_chain_id_for_public_testnet();
        let block_id = get_block_id_for_public_testnet();
        let address = get_address_for_fake_wallet_alice();

        let command = generate_boxed_get_balance_command(chain_id, block_id, address);
        let client = get_rpc_client();

        assert!(client.check_node_online().await);

        let raw_response = client.execute(&command).await;
        assert!(raw_response.is_ok());

        let response = raw_response.unwrap();
        assert_eq!(response.status(), 200);
    }

    fn get_rpc_client() -> RPCClient {
        // Public testnet as given here:
        // https://assets.tqtezos.com/docs/setup/1-tezos-client/#option-2--using-packages-on-ubuntu-or-fedora
        let tezos_node_url = get_local_testnet_url();
        RPCClient::new(tezos_node_url)
    }

    fn _get_public_testnet_url() -> Url {
        Url::parse("https://rpcalpha.tzbeta.net").unwrap()
    }

    fn get_local_testnet_url() -> Url {
        Url::parse("http://localhost:8090").unwrap()
    }

    fn get_chain_id_string() -> String {
        // NOTE: gets the ID string of the chain avaible on florencenet.
        // might change sometime.
        "NetXdQprcVkpaWU".to_string()
    }

    fn get_chain_id_for_public_testnet() -> String {
        // IMPORTANT: This is specifically for https://rpcalpha.tzbeta.net testnet
        "main".to_string()
    }

    fn get_block_id_for_public_testnet() -> String {
        // IMPORTANT: This is specifically for https://rpcalpha.tzbeta.net testnet
        "head".to_string()
    }

    fn get_address_for_fake_wallet_bob() -> String {
        // Address for a fake wallet belonging to Bob
        "tz1VkjQjvH3NTVdFtGz9tWutgxLKwnhnMe2x".to_string()
    }

    fn get_address_for_fake_wallet_alice() -> String {
        // Address for a fake wallet belonging to Alice
        "tz1PBRwGc83hSDBFw1LsyuzTcz1EGBHj4DMk".to_string()
    }

    fn generate_boxed_get_blocks_command(chain_id: String) -> Box<GetBlocksInChain> {
        Box::new(GetBlocksInChain { chain_id })
    }

    fn generate_boxed_get_balance_command(
        chain_id: String,
        block_id: String,
        address: String,
    ) -> Box<GetBalance> {
        Box::new(GetBalance {
            chain_id,
            block_id,
            address,
        })
    }
}
