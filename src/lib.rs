pub use client::{Client, ClientInfo};
pub use error::Error;
pub use std::{collections::HashMap, path};
pub use transaction::{TransactionInfo, TransactionType};

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
    /// Loads all records into memory at once.
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

    /// Stream records to save resources.
    /// This approach assumes that all transaction ids are guaranteed to be unique.
    pub async fn stream(&mut self, path: path::PathBuf) -> Result<(), Error> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_path(path)?;

        for result in rdr.into_deserialize() {
            let transaction_info: TransactionInfo = result?;
            // println!("result: {:?}", transaction_info);
            let transaction = self.transactions.get(&transaction_info.tx);

            // TODO: implement async
            let mut client = self
                .clients
                .entry(transaction_info.client)
                .or_insert_with_key(|_| ClientInfo::new(transaction_info.client));

            println!("\n\nt_type: {:?}", transaction_info.r#type);

            let transaction_type = TransactionType::try_from((
                transaction_info.r#type.as_str(),
                Some(transaction_info.amount).unwrap(),
            ))?;

            println!("transactions: {:?}", self.transactions);

            match transaction_type {
                TransactionType::Deposit(amount) => {
                    client.deposit(amount).await?;
                    self.transactions
                        .insert(transaction_info.tx, transaction_info.clone());
                }
                TransactionType::Withdrawal(amount) => {
                    client.withdraw(amount).await?;
                    self.transactions
                        .insert(transaction_info.tx, transaction_info.clone());
                }
                TransactionType::Dispute | TransactionType::Resolve => {
                    if transaction.is_some() {
                        println!("transaction is Some()");
                        client
                            .dispute_or_resolve(
                                &transaction_type,
                                transaction_info.tx,
                                transaction
                                    .ok_or(Error::UnableToProcessTransaction)?
                                    .amount
                                    .ok_or(Error::UnableToProcessAmount)?,
                            )
                            .await?
                    } else {
                        println!("Transaction ignored.");
                    }
                    // return Ok(())
                }
                TransactionType::Chargeback => {
                    client
                        .chargeback(
                            transaction
                                .ok_or(Error::UnableToProcessTransaction)?
                                .amount
                                .ok_or(Error::UnableToProcessAmount)?,
                        )
                        .await?
                }
                _ => return Err(Error::UnableToProcessTransaction),
            }

            for (k, v) in &self.clients {
                println!("{:?} - {:?}", k, v);
            }
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
