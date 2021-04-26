use super::Response;

pub struct BalanceResponse {
    pub balance: u32,
}

impl Response for BalanceResponse {
    fn from_response_str(response: &str) -> Self {
        let balance = response.trim().replace("\"", "").parse().unwrap();
        Self { balance }
    }
}
