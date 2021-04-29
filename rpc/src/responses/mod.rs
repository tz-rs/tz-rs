pub mod balance;
pub mod block_ids_in_chain;
mod bulk_array;
use crate::types::errors::ParseError;
pub use balance::BalanceResponse;
pub use block_ids_in_chain::BlocksInChainResponse;

pub trait Response {
    fn from_response_str(response: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}
