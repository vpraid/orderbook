//! The module contains the definition of a price level in an order book.
use crate::matcher::Matcher;
use crate::order::Order;
use crate::trade::Trade;
use crate::Price;

use std::collections::VecDeque;

/// Level represents a price level in an order book. The orders in a level are
/// placed in a deque for efficient processing by matching algorithms.
#[allow(dead_code)]
pub struct Level<'a> {
    price: Price,
    orders: VecDeque<Order<'a>>,
}

#[allow(dead_code)]
impl<'a> Level<'a> {
    /// Create a new price level.
    pub fn new(price: Price) -> Self {
        Self {
            price,
            orders: VecDeque::new(),
        }
    }

    /// Add an order to the level.
    pub fn add(&mut self, order: Order<'a>) {
        self.orders.push_front(order);
    }

    /// Cancel an order given by order ids.
    pub fn remove(&mut self, user_id: u64, user_order_id: u64) -> Option<Order<'a>> {
        for (idx, order) in self.orders.iter().enumerate() {
            if order.user_id() == user_id && order.user_order_id() == user_order_id {
                return self.orders.remove(idx);
            }
        }
        None
    }

    pub fn match_to<M: Matcher>(&mut self, other: &mut Self, matcher: &mut M) -> Vec<Trade<'a>> {
        debug_assert!(self.price == other.price);
        let mut trades = Vec::new();
        while let Some(mut order) = self.orders.pop_back() {
            trades.append(&mut matcher.match_order(&mut order, other));
            if order.is_valid() {
                self.orders.push_front(order);
                break;
            }
        }
        trades
    }

    /// Get the price of this level.
    pub fn price(&self) -> Price {
        self.price
    }

    pub fn orders(&self) -> &VecDeque<Order<'a>> {
        &self.orders
    }

    pub fn orders_mut(&mut self) -> &mut VecDeque<Order<'a>> {
        &mut self.orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::FIFOMatcher;
    use crate::order::{Order, Side};

    fn level_from_orders<'a>(orders: impl IntoIterator<Item = Order<'a>>) -> Level<'a> {
        let mut level = Level::new(1.0.into());
        for order in orders {
            level.add(order);
        }
        level
    }

    #[test]
    fn test_bid_less_than_ask() {
        let bid_orders = [
            Order::with_ids(1, 51).limit_order(Side::Bid, "AAPL", 1.0, 3),
            Order::with_ids(2, 52).limit_order(Side::Bid, "AAPL", 1.0, 7),
        ];
        let ask_orders = [
            Order::with_ids(10, 110).limit_order(Side::Ask, "AAPL", 1.0, 5),
            Order::with_ids(11, 111).limit_order(Side::Ask, "AAPL", 1.0, 10),
            Order::with_ids(12, 112).limit_order(Side::Ask, "AAPL", 1.0, 7),
        ];
        let mut bid_level = level_from_orders(bid_orders);
        let mut ask_level = level_from_orders(ask_orders);
        let trades = bid_level.match_to(&mut ask_level, &mut FIFOMatcher);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 3);
        assert_eq!(trades[0].user_id_buy, 1);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 10);
        assert_eq!(trades[0].user_order_id_sell, 110);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 2);
        assert_eq!(trades[1].user_id_buy, 2);
        assert_eq!(trades[1].user_order_id_buy, 52);
        assert_eq!(trades[1].user_id_sell, 10);
        assert_eq!(trades[1].user_order_id_sell, 110);
        assert_eq!(trades[2].price, Price::from(1.0));
        assert_eq!(trades[2].quantity, 5);
        assert_eq!(trades[2].user_id_buy, 2);
        assert_eq!(trades[2].user_order_id_buy, 52);
        assert_eq!(trades[2].user_id_sell, 11);
        assert_eq!(trades[2].user_order_id_sell, 111);

        // Remaining orders are correct
        assert_eq!(bid_level.orders().len(), 0);
        assert_eq!(ask_level.orders().len(), 2);
        assert_eq!(ask_level.orders()[0].quantity(), 7);
        assert_eq!(ask_level.orders()[1].quantity(), 5);
    }

    #[test]
    fn test_bid_more_than_ask() {
        let bid_orders = [
            Order::with_ids(1, 51).limit_order(Side::Bid, "AAPL", 1.0, 3),
            Order::with_ids(2, 52).limit_order(Side::Bid, "AAPL", 1.0, 12),
        ];
        let ask_orders = [
            Order::with_ids(10, 110).limit_order(Side::Ask, "AAPL", 1.0, 5),
            Order::with_ids(11, 111).limit_order(Side::Ask, "AAPL", 1.0, 7),
        ];
        let mut bid_level = level_from_orders(bid_orders);
        let mut ask_level = level_from_orders(ask_orders);
        let trades = bid_level.match_to(&mut ask_level, &mut FIFOMatcher);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 3);
        assert_eq!(trades[0].user_id_buy, 1);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 10);
        assert_eq!(trades[0].user_order_id_sell, 110);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 2);
        assert_eq!(trades[1].user_id_buy, 2);
        assert_eq!(trades[1].user_order_id_buy, 52);
        assert_eq!(trades[1].user_id_sell, 10);
        assert_eq!(trades[1].user_order_id_sell, 110);
        assert_eq!(trades[2].price, Price::from(1.0));
        assert_eq!(trades[2].quantity, 7);
        assert_eq!(trades[2].user_id_buy, 2);
        assert_eq!(trades[2].user_order_id_buy, 52);
        assert_eq!(trades[2].user_id_sell, 11);
        assert_eq!(trades[2].user_order_id_sell, 111);

        // Remaining orders are correct
        assert_eq!(bid_level.orders().len(), 1);
        assert_eq!(ask_level.orders().len(), 0);
        assert_eq!(bid_level.orders()[0].quantity(), 3);
    }
}
