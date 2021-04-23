use super::Response;

pub struct BalanceResponse {
    pub balance: i32,
}

impl Response for BalanceResponse {
    fn from_response_str(response: &str) -> Self {
        Self {
            balance: response.parse::<i32>().unwrap(),
        }
    }
}
