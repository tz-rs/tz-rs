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

pub mod commands;
pub mod errors;
pub mod responses;
pub mod types;
use commands::RpcClientCommand;
use url::Url;

/// Client wrapper and executor for making RPC calls to the Tezos net.
///
/// Execute commands implementing [`RpcClientCommand`] by passing them
/// into [`execute()`](Self::execute())
///
/// Should be instanciated only once and re-used so as to not reinstanciate
/// the inner `reqwest` client
pub struct RpcClient {
    tezos_node_url: Url,
    client: reqwest::Client,
}

impl RpcClient {
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
    /// [`command`](RpcClientCommand) passed in.
    ///
    /// Returns a response object parsed by the incoming command from the
    /// raw server response.
    pub async fn execute<T: RpcClientCommand>(
        &self,
        command: &T,
    ) -> Result<<T as RpcClientCommand>::R, errors::RpcError> {
        let raw_endpoint_url = format!("{}{}", self.tezos_node_url, command.get_url_string());
        let endpoint_url = reqwest::Url::parse(&raw_endpoint_url)?;
        println!("cont: {}", &endpoint_url);

        let request = self.client.request(command.get_http_method(), endpoint_url);
        let response_str = request.send().await?.text().await?;

        Ok(command.from_response_str(&response_str)?)
    }
}
