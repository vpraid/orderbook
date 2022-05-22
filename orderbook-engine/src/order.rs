//! This module contains the definition of all order structs that are used in the orderbook engine.

use super::Price;

/// Side represents the side of the order: bid or ask.
pub enum Side {
    Bid,
    Ask,
}

/// Order enum represents all possible order types that appear on the market. At present,
/// only limit orders are supported.
pub enum Order<'a> {
    Limit(LimitOrder<'a>),
    // TODO: add market orders and possibly other order types
}

impl Order<'_> {
    pub fn with_ids(user_id: u64, user_order_id: u64) -> OrderBuilder {
        OrderBuilder::new(user_id, user_order_id)
    }
}

/// Convenience struct for building orders.
pub struct OrderBuilder {
    user_id: u64,
    user_order_id: u64,
}

impl OrderBuilder {
    pub fn new(user_id: u64, user_order_id: u64) -> Self {
        Self {
            user_id,
            user_order_id,
        }
    }

    pub fn limit_order(self, side: Side, symbol: &str, price: f64, quantity: u64) -> Order {
        Order::Limit(LimitOrder {
            user_id: self.user_id,
            user_order_id: self.user_order_id,
            side,
            symbol,
            price: price.into(),
            quantity,
        })
    }
}

/// LimitOrder represet a limit order on the market. A limit order is a type of order to buy or sell
/// a security at a specific price or better.  If the side is 'bid', the price represents the maximum
/// price that a buyer is willing to pay for a share of stock or other security. If the side is 'ask',
/// the price represents the minimum price that a seller is willing to take for that same security.
pub struct LimitOrder<'a> {
    pub user_id: u64,
    pub user_order_id: u64,
    pub side: Side,
    pub symbol: &'a str,
    pub price: Price,
    pub quantity: u64,
}
