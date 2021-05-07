use crate::commands::RpcClientCommand;
use crate::responses::chains::current_checkpoint::CurrentCheckpointResponse;
use crate::types::Chain;

pub struct GetCurrentCheckpoint {
    pub chain_id: Chain,
}

impl RpcClientCommand for GetCurrentCheckpoint {
    type R = CurrentCheckpointResponse;

    fn get_url_string(&self) -> String {
        format!("chains/{}/checkpoint", &self.chain_id.to_str())
    }

    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
