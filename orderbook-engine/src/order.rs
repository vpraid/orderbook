//! This module contains the definition of all order structs that are used in the orderbook engine.

use crate::trade::Trade;
use crate::{Price, Symbol};

/// Side represents the side of the order: bid or ask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

/// Order enum represents all possible order types that appear on the market. At present,
/// only limit orders are supported.
#[derive(Debug)]
pub enum Order {
    Limit(LimitOrder),
    // TODO: add market orders and possibly other order types
}

impl Order {
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
    pub fn symbol(&self) -> Symbol {
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

    /// Check if this order is done, that is, if the quantitity is zero.
    pub fn is_done(&self) -> bool {
        self.quantity() == 0
    }

    pub fn match_to(&mut self, other: &mut Self) -> Trade {
        // This function will probably change significantly when new types of orders are introduced.
        // For now we just stick to the simplest possible implementation.
        match self {
            Order::Limit(order) => match other {
                Order::Limit(other_order) => order.match_to(other_order),
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

    pub fn limit_order(self, side: Side, symbol: Symbol, price: f64, quantity: u64) -> Order {
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
pub struct LimitOrder {
    pub user_id: u64,
    pub user_order_id: u64,
    pub side: Side,
    pub symbol: Symbol,
    pub price: Price,
    pub quantity: u64,
}

impl LimitOrder {
    pub fn match_to(&mut self, other: &mut Self) -> Trade {
        let (bid, ask) = match (self.side, other.side) {
            (Side::Bid, Side::Ask) => (self, other),
            (Side::Ask, Side::Bid) => (other, self),
            _ => panic!("Cannot trade with on the same side"),
        };
        debug_assert!(bid.symbol == ask.symbol, "Trade symbols don't match");
        debug_assert!(bid.price >= ask.price, "Bid must be greater than ask");

        let trade_quantity = bid.quantity.min(ask.quantity);
        bid.quantity -= trade_quantity;
        ask.quantity -= trade_quantity;

        Trade {
            user_id_buy: bid.user_id,
            user_order_id_buy: bid.user_order_id,
            user_id_sell: ask.user_id,
            user_order_id_sell: ask.user_order_id,
            symbol: bid.symbol,
            price: ask.price,
            quantity: trade_quantity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use string_interner::StringInterner;

    #[test]
    fn test_equal_trade_quantity() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, aapl, 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, aapl, 1.0, 10);
        let trade = bid_order.match_to(&mut ask_order);
        assert_eq!(trade.quantity, 10);
        assert_eq!(bid_order.quantity(), 0);
        assert_eq!(ask_order.quantity(), 0);
    }

    #[test]
    fn test_bid_quantity_higher() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, aapl, 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, aapl, 1.0, 7);
        let trade = bid_order.match_to(&mut ask_order);
        assert_eq!(trade.quantity, 7);
        assert_eq!(bid_order.quantity(), 3);
        assert_eq!(ask_order.quantity(), 0);
    }

    #[test]
    fn test_bid_quantity_lower() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, aapl, 1.0, 7);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, aapl, 1.0, 10);
        let trade = bid_order.match_to(&mut ask_order);
        assert_eq!(trade.quantity, 7);
        assert_eq!(bid_order.quantity(), 0);
        assert_eq!(ask_order.quantity(), 3);
    }

    #[test]
    #[should_panic]
    fn test_different_symbols() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let msft = si.get_or_intern_static("MSFT");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, aapl, 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, msft, 1.0, 10);
        bid_order.match_to(&mut ask_order);
    }

    #[test]
    #[should_panic]
    fn test_bid_price_lower() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, aapl, 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Ask, aapl, 2.0, 10);
        bid_order.match_to(&mut ask_order);
    }

    #[test]
    #[should_panic]
    fn test_ask_price_higher() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Ask, aapl, 2.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Bid, aapl, 1.0, 10);
        bid_order.match_to(&mut ask_order);
    }

    #[test]
    #[should_panic]
    fn tests_same_sides() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut bid_order = Order::with_ids(1, 101).limit_order(Side::Bid, aapl, 1.0, 10);
        let mut ask_order = Order::with_ids(2, 102).limit_order(Side::Bid, aapl, 1.0, 10);
        bid_order.match_to(&mut ask_order);
    }
}
