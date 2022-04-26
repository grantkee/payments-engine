use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error reading csv: {0}")]
    CsvError(#[from] csv::Error),
    #[error("Error with io: {0}")]
    IOError(#[from] io::Error),
    #[error("Unknown error has occurred.")]
    UnknownError,
}
