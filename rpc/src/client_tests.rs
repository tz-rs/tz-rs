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
