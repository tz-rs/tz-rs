use super::RPCClientCommand;
pub struct GetBlockInfo {
    pub chain_id: String,
    pub block_id: String,
}

impl RPCClientCommand for GetBlockInfo {
    fn get_url_string(&self) -> String {
        format!("chains/{}/blocks/{}", &self.chain_id, &self.block_id)
    }

    fn get_json_data(&self) -> Option<String> {
        None
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
