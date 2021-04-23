use super::RPCClientCommand;
use crate::responses::BlocksInChainResponse;

pub struct GetBlocksInChain {
    pub chain_id: String,
}

impl RPCClientCommand for GetBlocksInChain {
    type R = BlocksInChainResponse;

    fn get_url_string(&self) -> String {
        format!("chains/{}/blocks", &self.chain_id)
    }

    fn get_json_data(&self) -> Option<String> {
        None
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
