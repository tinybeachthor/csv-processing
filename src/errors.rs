//! Custom error types.

use std::io;
use csv;
use thiserror::Error;

/// Errors.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO Error : {error}")]
    Io {
        #[from]
        error: io::Error,
    },
    #[error("CSV Error : {error}")]
    Csv {
        #[from]
        error: csv::Error,
    },
    #[error("Expecting exactly 1 argument: path to the transactions file.")]
    WrongArguments(),
}
