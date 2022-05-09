use crate::transaction::TransactionType;
use serde::{Deserialize, Serialize, Serializer};

/// Unique struct.
///
/// Struct for writing to CSV.
#[derive(Debug, Serialize)]
pub struct Client {
    pub client: u16,
    #[serde(serialize_with = "serialize_f32")]
    pub available: f32,
    #[serde(serialize_with = "serialize_f32")]
    pub held: f32,
    #[serde(serialize_with = "serialize_f32")]
    pub total: f32,
    pub locked: bool,
}

/// Serialize the f32 values as rounded to 4 decimal places.
fn serialize_f32<S: Serializer>(num: &f32, s: S) -> std::result::Result<S::Ok, S::Error> {
    s.serialize_str(format!("{:.4}", num).as_str())
}

/// Unique struct.
///
/// Minimum amount of information needed for
/// managing client's transactions.
#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    pub id: u16,
    funds: ClientFunds,
    locked: bool,
    disputed_funds: Vec<u32>,
}

/// Struct for handling client's funds.
#[derive(Debug, Default, Deserialize)]
struct ClientFunds {
    // total is calculated when
    // creating Client from ClientInfo
    // total = available + held
    available: f32,
    held: f32,
}

/// Result alias that returns a custom Error.
type Result<T> = std::result::Result<T, crate::Error>;

impl ClientInfo {
    /// Create unique client with minimum amount
    /// of information needed to process all
    /// transactions.
    pub fn new(id: u16) -> Self {
        Self {
            id,
            funds: ClientFunds::default(),
            locked: false,
            disputed_funds: Vec::new(),
        }
    }

    /// Handle deposit transaction. Increase available (and thus total) funds.
    /// Do not process transaction if account is locked.
    pub async fn deposit(&mut self, amount: f32) -> Result<()> {
        if !self.locked {
            self.funds.available += amount;
        }
        Ok(())
    }

    /// Handle withdraw transactions. Decrease available (and thus total)
    /// funds, only if sufficient funds are available.
    /// Do not process transaction if account is locked, or the
    /// available funds are less than the amount to withdraw.
    pub async fn withdraw(&mut self, amount: f32) -> Result<()> {
        if !self.locked && self.funds.available >= amount {
            self.funds.available -= amount;
        }
        Ok(())
    }

    /// Handle dispute and resolve transactions.
    /// Keep track of disputed funds in ClientInfo.
    /// Locked accounts can still update disputed transactions.
    ///
    /// For disputes, hold associated funds and decrease available
    /// funds. Transactions must exist to call this method.
    ///
    /// For resolves, associated held funds are released. The
    /// transaction must exist and be under dispute to call this
    /// method.
    //
    // manually tested using dispute.csv, resolve.csv,
    // and wrong_client_dispute.csv
    pub async fn dispute_or_resolve(
        &mut self,
        t_type: &TransactionType,
        t_id: u32,
        amount: f32,
    ) -> Result<()> {
        match t_type {
            &TransactionType::Dispute => {
                self.funds.available -= amount;
                self.funds.held += amount;
                self.disputed_funds.push(t_id);
                Ok(())
            }
            &TransactionType::Resolve => {
                if let Some(index) = self.disputed_funds.iter().position(|val| *val == t_id) {
                    self.funds.available += amount;
                    self.funds.held -= amount;
                    self.disputed_funds.remove(index);
                }
                Ok(())
            }
            _ => Err(crate::Error::UnknownDisputeOrResolutionType),
        }
    }

    /// Handle client chargebacks.
    /// Lock the client's account if a chargeback occurs.
    pub async fn chargeback(&mut self, t_id: u32, amount: f32) -> Result<()> {
        // only process chargebacks for disputed transactions
        if let Some(_) = self.disputed_funds.iter().position(|val| *val == t_id) {
            self.locked = true;
            self.funds.held -= amount;
        }
        Ok(())
    }

    // only used in trait impl From<ClientInfo>
    // not async to prevent heal allocation
    /// Calculate total for Client based on
    /// available and held funds.
    fn get_total(&self) -> f32 {
        self.funds.held + self.funds.available
    }
}

/// Create the Client struct from ClientInfo.
/// Used to write back to CSV.
impl From<&ClientInfo> for Client {
    // do not want to use async-trait because
    // it results in heap allocation per-function-call
    fn from(info: &ClientInfo) -> Self {
        Self {
            client: info.id,
            available: info.funds.available,
            held: info.funds.held,
            total: info.get_total(),
            locked: info.locked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // New ClientInfo has correct defaults.
    #[tokio::test]
    async fn new_client_defaults() {
        let client = ClientInfo::new(1);
        assert_eq!(client.id, 1);
        assert_eq!(client.funds.available, 0.0);
        assert_eq!(client.funds.held, 0.0);
        assert_eq!(client.locked, false);
    }

    // Client created from ClientInfo struct
    #[tokio::test]
    async fn create_client_from_client_info() {
        let client_info = ClientInfo::new(1);
        let client = Client::from(&client_info);
        assert_eq!(client.client, 1);
        assert_eq!(client.available, 0.0);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.total, 0.0);
        assert_eq!(client.locked, false);
    }

    // client can make a deposit.
    #[tokio::test]
    async fn client_can_deposit() {
        let mut client_info = ClientInfo::new(1);
        let _deposit = client_info.deposit(1.0).await;
        assert_eq!(client_info.funds.available, 1.0)
    }

    // client can withdraw funds when the amount is there
    #[tokio::test]
    async fn client_can_withdraw() {
        let mut client_info = ClientInfo::new(1);
        let _deposit = client_info.deposit(1.0).await;
        let _withdraw = client_info.withdraw(0.02).await;
        assert_eq!(client_info.funds.available, 0.98);
        assert_eq!(client_info.funds.held, 0.0);
    }

    // client cannnot withdraw with insufficient funds
    #[tokio::test]
    async fn client_cannot_withdraw() {
        let mut client_info = ClientInfo::new(1);
        let _deposit = client_info.deposit(1.0).await;
        let _withdraw = client_info.withdraw(3.02).await;
        assert_eq!(client_info.funds.available, 1.0);
        assert_eq!(client_info.funds.held, 0.0);
    }

    // client cannot withdraw or deposet when account is locked
    #[tokio::test]
    async fn client_account_is_locked() {
        let mut client_info = ClientInfo::new(1);
        let _deposit = client_info.deposit(1.0006).await;
        let _dispute = client_info
            .dispute_or_resolve(&TransactionType::Dispute, 1, 1.0006)
            .await;
        let _chargeback = client_info.chargeback(1, 1.0006).await;
        let _withdraw_attempt = client_info.withdraw(0.03).await;
        let _deposit_attempt = client_info.deposit(0.01).await;
        assert_eq!(client_info.locked, true);
        assert_eq!(client_info.funds.available, 0.0);
        assert_eq!(client_info.funds.held, 0.0);
    }
}
