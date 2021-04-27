use super::RpcClientCommand;
use crate::responses::BlocksInChainResponse;
use crate::types::Chain;

pub struct GetBlocksInChain {
    pub chain_id: Chain,
}

impl RpcClientCommand for GetBlocksInChain {
    type R = BlocksInChainResponse;

    fn get_url_string(&self) -> String {
        format!("chains/{}/blocks", self.chain_id.to_str())
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
