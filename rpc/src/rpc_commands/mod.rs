mod get_balance;
mod get_blocks_in_chain;
pub use get_balance::GetBalance;
pub use get_blocks_in_chain::GetBlocksInChain;
use reqwest;

pub trait RPCClientCommand {
    fn get_url_string(&self) -> String;
    fn get_json_data(&self) -> Option<String>; // this wouldn't be a string (obviously) but a JsonValue later on
    fn get_http_method(&self) -> reqwest::Method;
}
