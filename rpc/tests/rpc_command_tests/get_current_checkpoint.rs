use super::*;
use commands::chains::get_current_checkpoint::GetCurrentCheckpoint;

#[tokio::test]
async fn get_current_checkpoint_ok() {
    let command = generate_get_checkpoint_command_for_main_chain();

    let client = get_rpc_client();
    assert!(client.check_node_online().await);

    let client_response = client.execute(&command).await;
    assert!(client_response.is_ok());

    let checkpoint = client_response.unwrap();
    assert!(checkpoint.save_point >= 0);
    assert!(checkpoint.caboose >= 0);
}

fn generate_get_checkpoint_command_for_main_chain() -> GetCurrentCheckpoint {
    let chain_id = get_main_chain_id_by_tag();
    GetCurrentCheckpoint { chain_id }
}
