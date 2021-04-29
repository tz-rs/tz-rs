use super::Response;
use std::num::ParseIntError;

pub struct BalanceResponse {
    pub balance: u32,
}

impl Response for BalanceResponse {
    type E = ParseIntError;
    fn from_response_str(response: &str) -> Result<Self, Self::E> {
        let balance = response.trim().replace("\"", "").parse()?;
        Ok(Self { balance })
    }
}
