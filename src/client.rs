use serde::Deserialize;

/// Unique struct.
///
/// Struct for writing to CSV.
#[derive(Debug)]
pub struct Client {
    pub id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

/// Unique struct.
///
/// Minimum amount of information needed for
/// managing client's transactions.
#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    id: u16,
    funds: ClientFunds,
    locked: bool,
}

/// Struct for handling client's funds.
#[derive(Debug, Default, Deserialize)]
struct ClientFunds {
    // total = available + held
    available: f64,
    held: f64,
}

impl ClientInfo {
    /// Create unique client with minimum amount
    /// of information needed to process all
    /// transactions.
    pub fn new(id: u16) -> Self {
        Self {
            id,
            funds: ClientFunds::default(),
            locked: false,
        }
    }

    pub async fn deposit(&mut self, amount: f64) {
        self.funds.available += amount
    }

    pub async fn withdraw(&mut self, amount: f64) {
        self.funds.available -= amount
    }

    pub async fn dispute(&mut self, amount: f64) {
        self.funds.available -= amount
    }

    // only used in trait impl From<ClientInfo>
    // not async to prevent heal allocation
    /// Calculate total for Client based on
    /// available and held funds.
    fn get_total(&self) -> f64 {
        self.funds.held + self.funds.available
    }
}

/// Create the Client struct from ClientInfo.
/// Used to write back to CSV.
impl From<ClientInfo> for Client {
    // do not want to use async-trait because
    // it results in heap allocation per-function-call
    fn from(info: ClientInfo) -> Self {
        Self {
            id: info.id,
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

    #[tokio::test]
    async fn new_client_defaults() {
        let client = ClientInfo::new(1);
        assert_eq!(client.id, 1);
        assert_eq!(client.funds.available, 0.0);
        assert_eq!(client.funds.held, 0.0);
        assert_eq!(client.locked, false);
    }

    #[tokio::test]
    async fn create_client_from_client_info() {
        let client_info = ClientInfo::new(1);
        let client = Client::from(client_info);
        assert_eq!(client.id, 1);
        assert_eq!(client.available, 0.0);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.total, 0.0);
        assert_eq!(client.locked, false);
    }
}
