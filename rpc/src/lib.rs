mod rpc_commands;
use reqwest;
use rpc_commands::RPCClientCommand;

pub struct RPCClient {
    main_url: String,
    client: reqwest::Client,
}

impl RPCClient {
    pub fn new(main_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { main_url, client }
    }

    pub async fn execute<T: RPCClientCommand>(&self, command: &Box<T>) -> reqwest::Result<String> {
        let raw_endpoint_url = format!("{}/{}", self.main_url, command.get_url_string(),);
        let endpoint_url = reqwest::Url::parse(&raw_endpoint_url).unwrap();

        let request = self.client.request(command.get_http_method(), endpoint_url);
        let response = request.send().await?.text().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod test_rpc_client {
    use super::rpc_commands::{GetBalance, GetBlocksInChain};
    use super::*;

    #[test]
    fn rpc_client_creation_ok() {
        let main_url = String::new();
        RPCClient::new(main_url);
    }

    #[tokio::test]
    async fn get_blocks_in_chain_ok() {
        let chain_id = get_chain_id_string();
        let command = generate_boxed_get_blocks_command(chain_id);

        let client = get_client();

        let raw_response = client.execute(&command).await;
        assert!(raw_response.is_ok());
        let _response = raw_response.unwrap();
    }

    #[tokio::test]
    async fn get_balance_for_bob_ok() {
        let chain_id = get_chain_id_for_public_testnet();
        let block_id = get_block_id_for_public_testnet();
        let address = get_address_for_fake_wallet_bob();

        let command = generate_boxed_get_balance_command(chain_id, block_id, address);
        let client = get_public_testnet_client();

        let raw_response = client.execute(&command).await;
        assert!(raw_response.is_ok());
        let response = raw_response.unwrap();
        print!("Balance: {}", response);
    }

    #[tokio::test]
    async fn get_balance_for_alice_ok() {
        let chain_id = get_chain_id_for_public_testnet();
        let block_id = get_block_id_for_public_testnet();
        let address = get_address_for_fake_wallet_alice();

        let command = generate_boxed_get_balance_command(chain_id, block_id, address);
        let client = get_public_testnet_client();

        let raw_response = client.execute(&command).await;
        assert!(raw_response.is_ok());
        let response = raw_response.unwrap();
        print!("Balance: {}", response);
    }

    fn get_client() -> RPCClient {
        let main_url = "http://localhost:8090".to_string();
        RPCClient::new(main_url)
    }

    fn get_public_testnet_client() -> RPCClient {
        // Public testnet as given here:
        // https://assets.tqtezos.com/docs/setup/1-tezos-client/#option-2--using-packages-on-ubuntu-or-fedora
        let main_url = "https://rpcalpha.tzbeta.net".to_string();
        RPCClient::new(main_url)
    }

    fn get_chain_id_string() -> String {
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
