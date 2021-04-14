use reqwest;
use std::io::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn test_ok() {
        assert_eq!(2, 2);
    }
}

enum TZChainType {
    Main,
    Test, // let x = TZChainType::Test("hello".to_string())
}

impl TZChainType {
    fn get_chain_type_string(self) -> String {
        match self {
            Self::Main => "main",
            Self::Test => "florencenet",
        }
        .to_string()
    }
}
struct RPCClient {
    main_url: String,
    chain_type: TZChainType,
    client: reqwest::Client,
}

impl RPCClient {
    fn execute(&self, command: &Box<dyn RPCClientCommand>) -> Result<()> {
        let endpoint_url = format!("{}/{}", self.main_url, command.get_url_name());
        let request = self.client.request(command.get_http_method(), endpoint_url);
        request.header("Content-Type", "application/json");
        Ok(())
    }
}

trait RPCClientCommand {
    fn get_url_name(&self) -> String;
    fn get_json_data(&self) -> Option<String>; // this wouldn't be a string (obviously) but a JsonValue later on
    fn get_http_method(&self) -> reqwest::Method;
}

struct GetBalance;

impl RPCClientCommand for GetBalance {
    fn get_url_name(&self) -> String {
        "root/chains/test/blocks/head/context/contracts/address/balance".to_string()
    }
    fn get_json_data(&self) -> Option<String> {
        None
    }
    fn get_http_method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
