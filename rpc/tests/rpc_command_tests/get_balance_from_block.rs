use super::*;
use commands::GetBalance;

#[tokio::test]
async fn get_balance_for_bob_ok() {
    let address = get_address_for_fake_wallet_bob();

    let command = generate_get_balance_command_for_public_testnet(address);
    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let raw_response = client.execute(&command).await;
    assert!(raw_response.is_ok());

    let response = raw_response.unwrap();
    assert_eq!(response.balance, 0);
}

#[tokio::test]
async fn get_balance_for_alice_ok() {
    let address = get_address_for_fake_wallet_alice();

    let command = generate_get_balance_command_for_public_testnet(address);
    let client = get_rpc_client();

    assert!(client.check_node_online().await);

    let raw_response = client.execute(&command).await;
    assert!(raw_response.is_ok());

    let response = raw_response.unwrap();
    assert_eq!(response.balance, 0);
}

fn get_address_for_fake_wallet_bob() -> String {
    // Address for a fake wallet belonging to Bob
    "tz1VkjQjvH3NTVdFtGz9tWutgxLKwnhnMe2x".to_string()
}

fn get_address_for_fake_wallet_alice() -> String {
    // Address for a fake wallet belonging to Alice
    "tz1PBRwGc83hSDBFw1LsyuzTcz1EGBHj4DMk".to_string()
}

fn generate_get_balance_command_for_public_testnet(address: String) -> GetBalance {
    let chain_id = get_main_chain_id_by_tag();
    let block_id = get_block_id_by_tag();
    GetBalance {
        chain_id,
        block_id,
        address,
    }
}
