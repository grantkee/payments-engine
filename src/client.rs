use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Client {
    id: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Client {
    pub async fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
