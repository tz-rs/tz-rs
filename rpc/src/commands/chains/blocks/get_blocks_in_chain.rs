use super::block_responses::BlocksInChainResponse;
use crate::commands::RpcClientCommand;
use crate::types::Chain;
use chrono::NaiveDateTime;

type BlockHash = String;

#[derive(Debug)]
pub struct GetBlocksInChain {
    pub chain_id: Chain,
    params: Option<GetBlocksInChainParameters>,
}

#[derive(Debug)]
struct GetBlocksInChainParameters {
    length: Option<u32>,
    head: Option<BlockHash>,
    min_date: Option<NaiveDateTime>,
}

impl GetBlocksInChainParameters {
    fn to_url_string(&self) -> String {
        let mut base_url = String::new();

        if let Some(length) = &self.length {
            base_url.push_str(&length.to_string());
        }
        if let Some(head) = &self.head {
            base_url.push_str(&head);
        }
        if let Some(min_date) = &self.min_date {
            base_url.push_str(&min_date.timestamp().to_string());
        }

        base_url
    }
}

impl GetBlocksInChain {
    pub fn with_default_params(chain_id: Chain) -> Self {
        Self {
            chain_id,
            params: None,
        }
    }

    pub fn with_explicit_params(
        chain_id: Chain,
        length: Option<u32>,
        head: Option<BlockHash>,
        min_date: Option<NaiveDateTime>,
    ) -> Self {
        let params = GetBlocksInChainParameters {
            length,
            head,
            min_date,
        };

        Self {
            chain_id,
            params: Some(params),
        }
    }
}

impl RpcClientCommand for GetBlocksInChain {
    type R = BlocksInChainResponse;

    fn get_url_string(&self) -> String {
        let mut url_string = format!("chains/{}/blocks", self.chain_id.to_str());
        if let Some(params) = &self.params {
            url_string.push('?');
            url_string.push_str(&params.to_url_string());
        }
        url_string
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
