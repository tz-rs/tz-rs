pub mod chains;
mod json_array;
use crate::errors::ParseError;
pub use chains::blocks::balance::BalanceResponse;
pub use chains::blocks::block_ids_in_chain::BlocksInChainResponse;

pub trait Response {
    fn from_response_str(response: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}
