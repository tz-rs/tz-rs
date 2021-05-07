use crate::errors::ParseError;
use crate::responses::Response;

#[derive(Debug)]
pub struct BalanceResponse {
    pub balance: u64,
}

impl Response for BalanceResponse {
    fn from_response_str(response: &str) -> Result<Self, ParseError> {
        let parse_response: serde_json::Value = {
            let sanitized_response = response.trim().replace("\"", "");
            serde_json::from_str(&sanitized_response)?
        };

        match parse_response.as_u64() {
            Some(balance) => Ok(Self { balance }),
            None => {
                let detail = format!("response is not a number. response: {}", response);
                Err(ParseError::ResponseParsingError(detail))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_balance_parse_ok() {
        let mock_balance = 10;
        let mock_response = mock_response_string_with_balance(mock_balance);

        let response_result = BalanceResponse::from_response_str(&mock_response);
        assert!(response_result.is_ok());

        let response = response_result.unwrap();
        assert_eq!(response.balance, mock_balance as u64);
    }

    #[test]
    fn test_negative_balance_invalid_fail() {
        let mock_balance = -10;
        let mock_response = mock_response_string_with_balance(mock_balance);

        let response_result = BalanceResponse::from_response_str(&mock_response);
        assert!(response_result.is_err());
    }

    #[test]
    fn test_empty_string_fail() {
        let mock_response = "";

        let response_result = BalanceResponse::from_response_str(&mock_response);
        assert!(response_result.is_err());
    }

    fn mock_response_string_with_balance(mock_balance: i64) -> String {
        format!(r#""{}""#, mock_balance)
    }
}
