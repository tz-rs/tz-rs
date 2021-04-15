use super::RPCClientCommand;
use reqwest;
pub struct GetBalance;

impl RPCClientCommand for GetBalance {
    fn get_url_string(&self) -> String {
        "".to_string()
    }
    fn get_json_data(&self) -> Option<String> {
        None
    }
    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
