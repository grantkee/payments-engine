use serde::Deserialize;

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInfo {
    #[serde(rename = "type")]
    transaction: String,
    client: u16,
    tx: u32,
    amount: f64,
}

