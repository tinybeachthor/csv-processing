//! Custom error types.

use thiserror::Error;

/// Config errors.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Client {0} : balance too low for withdrawal {1}.")]
    BalanceLowForWithdrawal(u16, u32),
}
