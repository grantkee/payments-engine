pub use error::Error;
pub use std::{path, collections};
pub use transaction::TransactionInfo;
pub use client::Client;

mod error;
mod transaction;
mod client;

#[derive(Debug, Default)]
pub struct Engine {
    // consider using BTreeMap if sorting by ids
    pub transactions: collections::HashMap<u32, TransactionInfo>,
    pub clients: collections::HashMap<u16, Client>
}

impl Engine {
    /// Read the CSV file from the argument provided to CLI.
    pub async fn read(&self, path: path::PathBuf) -> Result<(), Error> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(path)?;
    
        for result in rdr.deserialize() {
            let record: TransactionInfo = result?;
            println!("{:?}", record);
        }
    
        Ok(())
    }

    /// Write to stdout
    pub async fn write(&self) -> Result<(), Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_handle_whitespace() {
        let engine = Engine::default();
        let result = engine.read(path::PathBuf::from("./tests/CSVs/transactions.csv")).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn correct_path_must_exist() {
        let engine = Engine::default();
        let result = engine.read(path::PathBuf::from("./tests/NONEXISTENT_PATH.csv")).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn new_engine_defaults() {
        let engine = Engine::default();
        assert!(engine.transactions.is_empty());
        assert!(engine.clients.is_empty());
    }
}
