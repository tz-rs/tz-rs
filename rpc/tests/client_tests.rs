mod rpc_command_tests;

use rpc::RPCClient;
use url::Url;

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
