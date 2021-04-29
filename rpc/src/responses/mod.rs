pub mod balance;
pub mod block_ids_in_chain;
mod bulk_array;
pub use balance::BalanceResponse;
pub use block_ids_in_chain::BlocksInChainResponse;
use std::error::Error;

pub trait Response {
    type E;
    fn from_response_str(response: &str) -> Result<Self, Self::E>
    where
        Self: Sized,
        Self::E: Error + Sized;
}
