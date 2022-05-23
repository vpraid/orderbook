//! This module contains the definition of a market trade.

use crate::{Price, Symbol};

/// Trade represents a successful trade transaction on the market.
#[derive(Debug)]
pub struct Trade {
    pub user_id_buy: u64,
    pub user_order_id_buy: u64,
    pub user_id_sell: u64,
    pub user_order_id_sell: u64,
    pub symbol: Symbol,
    pub price: Price,
    pub quantity: u64,
}
