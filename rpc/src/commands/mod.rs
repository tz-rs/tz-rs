use crate::errors::ParseError;
use crate::responses::Response;

pub mod chains;

pub trait RpcClientCommand {
    type R: Response;

    fn get_url_string(&self) -> String;
    fn get_http_method(&self) -> reqwest::Method;
    fn from_response_str(&self, string: &str) -> Result<Self::R, ParseError> {
        Self::R::from_response_str(string)
    }
}
