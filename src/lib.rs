pub use client::{Client, ClientInfo};
pub use error::Error;
pub use std::{collections::HashMap, io, path};
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
    /// Stream records to save resources.
    /// This approach assumes that all transaction ids are guaranteed to be unique.
    pub async fn stream(&mut self, path: path::PathBuf) -> Result<(), Error> {
        let rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .flexible(true)
            .from_path(path)?;

        for result in rdr.into_deserialize() {
            let transaction_info: TransactionInfo = result?;
            let transaction = self.transactions.get(&transaction_info.tx);

            // TODO: implement async
            let client = self
                .clients
                .entry(transaction_info.client)
                .or_insert_with_key(|_| ClientInfo::new(transaction_info.client));

            let transaction_type = TransactionType::try_from((
                transaction_info.r#type.as_str(),
                Some(transaction_info.amount).unwrap(),
            ))?;

            match transaction_type {
                // only keep records for deposits and withdrawals in engine struct.
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
                        let transaction = transaction.ok_or(Error::UnableToProcessTransaction)?;
                        // ensure disputed/resolved transactions belong to the client
                        // or else ignore. Assuming only a client can dispute a transaction
                        // for their own account.
                        if transaction.client == client.id {
                            client
                            .dispute_or_resolve(
                                &transaction_type,
                                transaction_info.tx,
                                transaction
                                .amount
                                .ok_or(Error::UnableToProcessAmount)?,
                            )
                            .await?
                        }
                    }
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
            }
        }

        Ok(())
    }

    /// Write serialized clients to stdout
    pub async fn write(&self) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        for client in self.clients.iter().map(|(_, client)| Client::from(client)) {
            wtr.serialize(client)?;
        }

        wtr.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_handle_whitespace() {
        let mut engine = Engine::default();
        let result = engine
            .stream(path::PathBuf::from("./tests/CSVs/whitespace.csv"))
            .await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn correct_path_must_exist() {
        let mut engine = Engine::default();
        let result = engine
            .stream(path::PathBuf::from("./tests/NONEXISTENT_PATH.csv"))
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
            .stream(path::PathBuf::from("./tests/unknown_transaction_type.csv"))
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn fail_to_overdraft() {
        let mut engine = Engine::default();
        let result = engine
            .stream(path::PathBuf::from("./tests/overdraft.csv"))
            .await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn duplicate_transactions_fail() {
        let mut engine = Engine::default();
        let result = engine
            .stream(path::PathBuf::from("./tests/duplicate_transactions.csv"))
            .await;
        assert!(result.is_err())
    }
}
