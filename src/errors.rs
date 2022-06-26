//! Custom error types.

use thiserror::Error;

/// Errors.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Client {0} : balance too low for withdrawal {1}.")]
    BalanceLowForWithdrawal(u16, u32),
    #[error("Client {0} : duplicate transaction id {1}.")]
    DuplicateTransaction(u16, u32),
}
