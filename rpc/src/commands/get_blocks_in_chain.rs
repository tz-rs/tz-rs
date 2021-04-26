use super::RPCClientCommand;

enum ChainType {
    Main,
    Other(String),
}

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
    fn get_url_string(&self) -> String {
        let chain_id_string = match &self.chain_id {
            ChainType::Main => "main",
            ChainType::Other(chain_id) => chain_id,
        };
        format!("chains/{}/blocks", &chain_id_string)
    }

    fn get_json_data(&self) -> Option<String> {
        None
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
