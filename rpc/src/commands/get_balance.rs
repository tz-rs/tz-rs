use super::RPCClientCommand;
use crate::responses::BalanceResponse;
use crate::types::ChainType;

pub struct GetBalance {
    pub chain_id: ChainType,
    pub block_id: String,
    pub address: String,
}

impl RPCClientCommand for GetBalance {
    type R = BalanceResponse;

    fn get_url_string(&self) -> String {
        format!(
            "chains/{}/blocks/{}/context/contracts/{}/balance",
            &self.chain_id.to_str(),
            &self.block_id,
            &self.address
        )
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
