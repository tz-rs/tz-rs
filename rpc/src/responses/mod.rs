pub mod chains;
pub mod json_array;
use crate::errors::ParseError;

pub trait Response {
    fn from_response_str(response: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}
