//! This module contains the definition of all order structs that are used in the orderbook engine.

use crate::trade::Trade;
use crate::Price;

/// Side represents the side of the order: bid or ask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

/// Order enum represents all possible order types that appear on the market. At present,
/// only limit orders are supported.
#[derive(Debug)]
pub enum Order<'a> {
    Limit(LimitOrder<'a>),
    // TODO: add market orders and possibly other order types
}

impl<'a> Order<'a> {
    /// Start building an order with the given user and order ids.
    pub fn with_ids(user_id: u64, user_order_id: u64) -> OrderBuilder {
        OrderBuilder::new(user_id, user_order_id)
    }

    /// Get the user id of the order.
    pub fn user_id(&self) -> u64 {
        match self {
            Order::Limit(order) => order.user_id,
        }
    }

    /// Get the user order id of the order.
    pub fn user_order_id(&self) -> u64 {
        match self {
            Order::Limit(order) => order.user_order_id,
        }
    }

    /// Get the order symbol.
    pub fn symbol(&self) -> &'a str {
        match self {
            Order::Limit(order) => order.symbol,
        }
    }

    /// Get the order bid or ask price.
    pub fn price(&self) -> Price {
        match self {
            Order::Limit(order) => order.price,
        }
    }

    /// Get the order side.
    pub fn side(&self) -> Side {
        match self {
            Order::Limit(order) => order.side,
        }
    }

    /// Get the order quantity.
    pub fn quantity(&self) -> u64 {
        match self {
            Order::Limit(order) => order.quantity,
        }
    }

    /// Check if this order is still valid, that is, if the quantitity is not zero.
    pub fn is_valid(&self) -> bool {
        self.quantity() > 0
    }

    pub fn trade(&mut self, other: &mut Self) -> Trade<'a> {
        // This function will probably change significantly when new types of orders are introduced.
        // For now we just stick to the simplest possible implementation.
        match self {
            Order::Limit(order) => match other {
                Order::Limit(other_order) => order.trade(other_order),
            },
        }
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
#[derive(Debug)]
pub struct LimitOrder<'a> {
    pub user_id: u64,
    pub user_order_id: u64,
    pub side: Side,
    pub symbol: &'a str,
    pub price: Price,
    pub quantity: u64,
}

impl<'a> LimitOrder<'a> {
    pub fn trade(&mut self, other: &mut Self) -> Trade<'a> {
        debug_assert!(self.side != other.side, "Cannot trade with the same side");
        debug_assert!(self.symbol == other.symbol, "Trade symbols don't match");
        debug_assert!(self.price >= other.price, "Trade prices don't match");
        let trade_quantity = self.quantity.min(other.quantity);
        self.quantity -= trade_quantity;
        other.quantity -= trade_quantity;
        let (user_id_buy, user_id_sell) = self.user_ids(other);
        let (user_order_id_buy, user_order_id_sell) = self.user_order_ids(other);
        Trade {
            user_id_buy,
            user_order_id_buy,
            user_id_sell,
            user_order_id_sell,
            symbol: self.symbol,
            price: self.price,
            quantity: trade_quantity,
        }
    }

    fn user_ids(&self, other: &Self) -> (u64, u64) {
        match self.side {
            Side::Bid => (self.user_id, other.user_id),
            Side::Ask => (other.user_id, self.user_id),
        }
    }

    fn user_order_ids(&self, other: &Self) -> (u64, u64) {
        match self.side {
            Side::Bid => (self.user_order_id, other.user_order_id),
            Side::Ask => (other.user_order_id, self.user_order_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_trade_quantity() {
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, "AAPL", 1.0, 10);
        let trade = bid_order.trade(&mut ask_order);
        assert_eq!(trade.quantity, 10);
        assert_eq!(bid_order.quantity(), 0);
        assert_eq!(ask_order.quantity(), 0);
    }

    #[test]
    fn test_bid_quantity_higher() {
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, "AAPL", 1.0, 7);
        let trade = bid_order.trade(&mut ask_order);
        assert_eq!(trade.quantity, 7);
        assert_eq!(bid_order.quantity(), 3);
        assert_eq!(ask_order.quantity(), 0);
    }

    #[test]
    fn test_bid_quantity_lower() {
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 1.0, 7);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, "AAPL", 1.0, 10);
        let trade = bid_order.trade(&mut ask_order);
        assert_eq!(trade.quantity, 7);
        assert_eq!(bid_order.quantity(), 0);
        assert_eq!(ask_order.quantity(), 3);
    }

    #[test]
    #[should_panic]
    fn test_different_symbols() {
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, "MSFT", 1.0, 10);
        bid_order.trade(&mut ask_order);
    }

    #[test]
    #[should_panic]
    fn test_different_price_levels() {
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, "AAPL", 2.0, 10);
        bid_order.trade(&mut ask_order);
    }

    #[test]
    #[should_panic]
    fn tests_same_sides() {
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Bid, "AAPL", 1.0, 10);
        bid_order.trade(&mut ask_order);
    }
}
