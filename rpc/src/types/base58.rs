use crate::errors::ParseError;
use bs58;

pub struct Base58 {
    base58_string: String,
}

impl Base58 {
    pub fn Base58Check(&self) -> Result<&str, ParseError> {
        let invaid_base58_error = ParseError::RequestParsingError("invalid base58 encoded string");

        match bs58::decode(self.base58_string).into_vec()? {
            Ok => self.base58_string,
            Err => invaid_base58_error,
        }
    }
}
