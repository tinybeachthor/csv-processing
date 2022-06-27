#![forbid(unsafe_code, future_incompatible)]
#![forbid(rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

//! CSV processor.

mod four_decimals;
pub use four_decimals::FourDecimals;

mod transaction;
pub use transaction::{Transaction, TransactionType};

mod client;
pub use client::Client;

mod errors;
pub use errors::MyError;

mod clients;
pub use clients::Clients;
