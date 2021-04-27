use super::RPCClientCommand;
use crate::responses::BlocksInChainResponse;
use crate::types::ChainType;

pub struct GetBlocksInChain {
    chain_id: ChainType,
}

impl GetBlocksInChain {
    pub fn with_chain_id(chain_id_string: String) -> Self {
        let chain_id = ChainType::Other(chain_id_string);
        Self { chain_id }
    }

    pub fn for_main_chain() -> Self {
        let chain_id = ChainType::Main;
        Self { chain_id }
    }
}

impl RPCClientCommand for GetBlocksInChain {
    type R = BlocksInChainResponse;

    fn get_url_string(&self) -> String {
        format!("chains/{}/blocks", self.chain_id.to_string())
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
