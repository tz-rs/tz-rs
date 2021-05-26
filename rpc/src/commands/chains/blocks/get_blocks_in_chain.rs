use super::block_responses::BlocksInChainResponse;
use crate::commands::RpcClientCommand;
use crate::types::Chain;
use chrono::NaiveDateTime;
use querystring;

type BlockHash = String;

#[derive(Debug)]
/// Command for the [`/chains/{chain_id}/blocks` endpoint](https://tezos.gitlab.io/shell/rpc.html#get-chains-chain-id-blocks).
///
/// Requires a valid [`chain_id`](Chain) to form the URL string
///
/// Can optionally hold extra query parameters with
/// [an explicit constructor](Self::with_explicit_params),
/// or default to sending no args with [the default constructor](Self::with_default_params)
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
    fn to_url_query_string(&self) -> String {
        let mut query_pairs = Vec::new();

        if let Some(length) = &self.length {
            query_pairs.push(("length", length.to_string()));
        }
        if let Some(head) = &self.head {
            query_pairs.push(("head", head.to_string()));
        }
        if let Some(min_date) = &self.min_date {
            query_pairs.push(("min_date", min_date.timestamp().to_string()));
        }

        let query_params = query_pairs.iter().map(|x| (x.0, x.1.as_str())).collect();

        querystring::stringify(query_params)
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
            url_string.push_str(&params.to_url_query_string());
        }
        url_string
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
