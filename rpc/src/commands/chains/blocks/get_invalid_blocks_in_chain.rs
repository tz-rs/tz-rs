use super::block_responses::InvalidBlocksInChainResponse;
use crate::commands::RpcClientCommand;
use crate::types::Chain;

pub struct GetInvalidBlocksInChain {
    pub chain_id: Chain,
}

impl RpcClientCommand for GetInvalidBlocksInChain {
    type R = InvalidBlocksInChainResponse;

    fn get_url_string(&self) -> String {
        format!("chains/{}/invalid_blocks", self.chain_id.to_str())
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
