//! This module contains the definition of a market trade.

use crate::Price;

/// Trade represents a successful trade transaction on the market.
#[derive(Debug)]
pub struct Trade<'a> {
    pub user_id_buy: u64,
    pub user_order_id_buy: u64,
    pub user_id_sell: u64,
    pub user_order_id_sell: u64,
    pub symbol: &'a str,
    pub price: Price,
    pub quantity: u64,
}
