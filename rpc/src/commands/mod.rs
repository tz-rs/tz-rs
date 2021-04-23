use crate::responses::Response;
mod get_balance;
mod get_blocks_in_chain;
pub use get_balance::GetBalance;
pub use get_blocks_in_chain::GetBlocksInChain;

pub trait RPCClientCommand {
    type R: Response;
    fn get_url_string(&self) -> String;
    fn get_json_data(&self) -> Option<String>; // this wouldn't be a string (obviously) but a JsonValue later on
    fn get_http_method(&self) -> reqwest::Method;
    fn from_response_str(&self, string: &str) -> Self::R {
        Self::R::from_response_str(string)
    }
}
