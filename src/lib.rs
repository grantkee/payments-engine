pub use client::{Client, ClientInfo};
pub use error::Error;
pub use std::{collections::HashMap, path};
pub use transaction::TransactionInfo;

mod client;
mod error;
mod transaction;

#[derive(Debug, Default)]
pub struct Engine {
    // consider using BTreeMap if sorting by ids
    pub transactions: HashMap<u32, TransactionInfo>,
    pub clients: HashMap<u16, ClientInfo>,
}

impl Engine {
    /// Read the CSV file from the argument provided to CLI.
    pub async fn read(&mut self, path: path::PathBuf) -> Result<(), Error> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(path)?;

        for result in rdr.deserialize() {
            let transaction: TransactionInfo = result?;
            println!("{:?}", transaction);

            // let mut client = self
            //     .clients
            //     .entry(transaction.client)
            //     .or_insert_with(|| Box::pin(ClientInfo::new(transaction.client).await));

            // TODO: implement async
            let mut client = self
                .clients
                .entry(transaction.client)
                .or_insert_with_key(|_| ClientInfo::new(transaction.client));

            if self.transactions.contains_key(&transaction.tx) {
                // do not process transactions more than once
                println!("Transaction {:?} already exists", transaction.tx);
                return Err(Error::TransactionAlreadyProcessed(transaction.tx));
            } else {
                match transaction.r#type.as_str() {
                    "deposit" => println!("transaction is deposit"),
                    "withdrawal" => println!("transaction is withdrawal"),
                    "dispute" => println!("transaction is a dispute"),
                    "resolve" => println!("transaction is a resolve"),
                    "chargeback" => println!("transaction is a chargeback"),
                    _ => return Err(Error::UnknownTransactionType(transaction.r#type)),
                }
            }
        }

        for (k, v) in &self.clients {
            println!("{:?} - {:?}", k, v);
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
        let mut engine = Engine::default();
        let result = engine
            .read(path::PathBuf::from("./tests/CSVs/whitespace.csv"))
            .await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn correct_path_must_exist() {
        let mut engine = Engine::default();
        let result = engine
            .read(path::PathBuf::from("./tests/NONEXISTENT_PATH.csv"))
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn new_engine_defaults() {
        let mut engine = Engine::default();
        assert!(engine.transactions.is_empty());
        assert!(engine.clients.is_empty());
    }

    #[tokio::test]
    async fn unknown_transaction_type_fails() {
        let mut engine = Engine::default();
        let result = engine
            .read(path::PathBuf::from("./tests/unknown_transaction_type.csv"))
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn duplicate_transactions_fail() {
        let mut engine = Engine::default();
        let result = engine
            .read(path::PathBuf::from("./tests/duplicate_transactions.csv"))
            .await;
        assert!(result.is_err())
    }
}
