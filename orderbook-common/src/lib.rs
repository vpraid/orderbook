use serde::{Deserialize, Serialize};

pub const SOCKET: &str = "/tmp/orderbook-server.sock";

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    New(NewOrder),
    Cancel(CancelOrder),
    Flush,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOrder {
    pub user_id: u64,
    pub user_order_id: u64,
    pub symbol: String,
    pub price: u64,
    pub quantity: u64,
    pub side: char,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    pub user_id: u64,
    pub user_order_id: u64,
}
