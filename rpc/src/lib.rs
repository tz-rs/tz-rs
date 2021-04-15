mod rpc_commands;
use reqwest;
use rpc_commands::{GetBlocksInChain, RPCClientCommand};

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
        let response = raw_response.unwrap();
    }

    fn get_client() -> RPCClient {
        let main_url = "http://localhost:8090".to_string();
        RPCClient::new(main_url)
    }

    fn get_chain_id_string() -> String {
        "NetXdQprcVkpaWU".to_string()
    }

    fn generate_boxed_get_blocks_command(chain_id: String) -> Box<GetBlocksInChain> {
        Box::new(GetBlocksInChain { chain_id })
    }
}
