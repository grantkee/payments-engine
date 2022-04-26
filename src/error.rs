use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error reading csv: {0}")]
    CsvError(#[from] csv::Error),
    #[error("Error with io: {0}")]
    IOError(#[from] io::Error),
    #[error("Error from duplicate transaction: {0}")]
    TransactionAlreadyProcessed(u32),
    #[error("Unknown transaction type: {0}")]
    UnknownTransactionType(String),
    #[error("Unknown error has occurred.")]
    UnknownError,
}
