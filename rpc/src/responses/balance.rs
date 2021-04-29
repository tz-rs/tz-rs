use super::ParseError;
use super::Response;

pub struct BalanceResponse {
    pub balance: u32,
}

impl Response for BalanceResponse {
    fn from_response_str(response: &str) -> Result<Self, ParseError> {
        let balance = response.trim().replace("\"", "").parse()?;
        Ok(Self { balance })
    }
}
