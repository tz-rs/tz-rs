use super::RpcClientCommand;
use crate::responses::BalanceResponse;
use crate::types::{Block, Chain};

pub struct GetBalance {
    pub chain_id: Chain,
    pub block_id: Block,
    pub address: String,
}

impl RpcClientCommand for GetBalance {
    type R = BalanceResponse;

    fn get_url_string(&self) -> String {
        format!(
            "chains/{}/blocks/{}/context/contracts/{}/balance",
            &self.chain_id.to_str(),
            &self.block_id.to_str(),
            &self.address
        )
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
