use super::RPCClientCommand;
pub struct GetBalance {
    pub chain_id: String,
    pub block_id: String,
    pub address: String,
}

impl RPCClientCommand for GetBalance {
    fn get_url_string(&self) -> String {
        format!(
            "chains/{}/blocks/{}/context/contracts/{}/balance",
            &self.chain_id, &self.block_id, &self.address
        )
    }

    fn get_json_data(&self) -> Option<String> {
        None
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
