use serde::Deserialize;

/// Defines the type of transaction.
pub enum TransactionType {
    /// A deposit is a credit to the client's asset account,
    /// and increases the available and total funds of the
    /// client account.
    Deposit(f64),

    /// A withdraw is a debit to the client's asset account,
    /// and decreases the available and total funds of the
    /// client account.
    Withdrawal(f64),

    /// A dispute represents a client's claim that a
    /// transaction was erroneous and should be reversed.
    /// The available funds decrease by the amount disputed,
    /// their held funds increase by the amount disputed,
    /// and their total funds remain the same.
    Dispute,

    /// A resolve represents a resolution to a dispute,
    /// releasing the associated held funds. Funds that
    /// were previously disputed are no longer disputed,
    /// The client's held funds decrease by the amount
    /// no longer disputed, their available funds increase
    /// by the amount no longer disputed, and the total funds
    /// remain the same.
    Resolve,

    /// A chargeback is the final state of a dispute and
    /// represents the client reversing a transaction. Funds
    /// that were held have now been withdrawn. The client's
    /// held funds and total funds decrease by the amount
    /// previously disputed and their account is frozen.
    Chargeback,
}

/// Information regarding the transaction. Includes the
/// transaction type, the client's id, the transaction's
/// id, and the amount of the transaction.
#[derive(Debug, Deserialize)]
pub struct TransactionInfo {
    r#type: String,
    client: u16,
    tx: u32,
    amount: f64,
}

