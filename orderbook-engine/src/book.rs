//! This module contains the definition of an order book - the primary structure for trading on the market.

use crate::level::Level;
use crate::market::Index;
use crate::matcher::Matcher;
use crate::order::{Order, Side};
use crate::trade::Trade;
use crate::Price;

use std::collections::BTreeMap;

type LevelMap = BTreeMap<Price, Level>;

/// Book represents an order book. It is implemented as a collection of levels for bid and for ask
/// orders separately. When a match must be done, the top level from the bids collection and the
/// bottom level from the asks collection are matched according to a matching algorithm.
pub struct Book {
    bids: LevelMap,
    asks: LevelMap,
}

impl Book {
    /// Create a new order book.
    pub fn new() -> Self {
        Self {
            bids: LevelMap::new(),
            asks: LevelMap::new(),
        }
    }

    /// Add an order to the book.
    pub fn add<M: Matcher>(&mut self, mut order: Order, matcher: &mut M) -> (bool, Vec<Trade>) {
        let trades = self.try_execute(&mut order, matcher);
        if order.is_done() {
            return (false, trades);
        }
        let price = order.price();
        let level = match order.side() {
            Side::Bid => self
                .bids
                .entry(price)
                .or_insert_with(|| Level::new(price, Side::Bid)),
            Side::Ask => self
                .asks
                .entry(-price)
                .or_insert_with(|| Level::new(price, Side::Ask)),
        };
        level.add(order);
        (true, trades)
    }

    /// Cancel an given order, removing it from the order book immediately.
    pub fn remove(&mut self, index: &Index) -> Option<Order> {
        let level = match index.side {
            Side::Bid => self.bids.get_mut(&index.price)?,
            Side::Ask => self.asks.get_mut(&-index.price)?,
        };
        level.remove(index.user_id, index.user_order_id)
    }

    /// Try executing the order.
    fn try_execute<M: Matcher>(&mut self, order: &mut Order, matcher: &mut M) -> Vec<Trade> {
        let levels = match order.side() {
            Side::Bid => &mut self.asks,
            Side::Ask => &mut self.bids,
        };
        let mut trades = Vec::new();
        while !levels.is_empty() {
            let (&key, top_level) = levels.iter_mut().next_back().unwrap();
            match order.side() {
                Side::Bid if order.price() < top_level.price() => break,
                Side::Ask if order.price() > top_level.price() => break,
                _ => (),
            }
            trades.append(&mut matcher.match_order(order, top_level));
            if top_level.is_empty() {
                levels.remove(&key);
            }
            if order.is_done() {
                break;
            }
        }
        trades
    }

    /// Clear this order book of all orders.
    pub fn clear(&mut self) {
        self.bids.clear();
        self.asks.clear();
    }

    pub fn top_of_book(&self, side: Side) -> (Price, u64) {
        let order = match side {
            Side::Bid => self
                .bids
                .iter()
                .next_back()
                .and_then(|(_, level)| level.top()),
            Side::Ask => self
                .asks
                .iter()
                .next_back()
                .and_then(|(_, level)| level.top()),
        }
        .expect("Order book is empty");
        (order.price(), order.quantity())
    }
}

impl Default for Book {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::FIFOMatcher;
    use crate::order::{Order, Side};
    use string_interner::StringInterner;

    fn book_from_orders(orders: impl IntoIterator<Item = Order>) -> Book {
        let mut book = Book::new();
        for order in orders {
            book.add(order, &mut FIFOMatcher);
        }
        book
    }

    #[test]
    fn test_cross_market_orders_from_bid() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let ask_orders = [
            Order::with_ids(10, 110).limit_order(Side::Ask, aapl, 1.0, 5),
            Order::with_ids(11, 111).limit_order(Side::Ask, aapl, 2.0, 5),
        ];
        let mut book = book_from_orders(ask_orders);
        let order = Order::with_ids(2, 52).limit_order(Side::Bid, aapl, 2.0, 7);
        let trades = book.add(order, &mut FIFOMatcher).1;

        // Trades are correct
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, 1.0);
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 2);
        assert_eq!(trades[0].user_order_id_buy, 52);
        assert_eq!(trades[0].user_id_sell, 10);
        assert_eq!(trades[0].user_order_id_sell, 110);
        assert_eq!(trades[1].price, 2.0);
        assert_eq!(trades[1].quantity, 2);
        assert_eq!(trades[1].user_id_buy, 2);
        assert_eq!(trades[1].user_order_id_buy, 52);
        assert_eq!(trades[1].user_id_sell, 11);
        assert_eq!(trades[1].user_order_id_sell, 111);

        // Remaining order are correct
        assert_eq!(book.bids.len(), 0);
        assert_eq!(book.asks.len(), 1);
        assert_eq!(book.asks.iter().next().unwrap().1.orders()[0].quantity(), 3);
    }

    #[test]
    fn test_cross_market_orders_from_ask() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let bid_orders = [
            Order::with_ids(10, 110).limit_order(Side::Bid, aapl, 1.0, 5),
            Order::with_ids(11, 111).limit_order(Side::Bid, aapl, 2.0, 5),
        ];
        let mut book = book_from_orders(bid_orders);
        let order = Order::with_ids(2, 52).limit_order(Side::Ask, aapl, 1.0, 7);
        let trades = book.add(order, &mut FIFOMatcher).1;

        // Trades are correct
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, 1.0);
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 11);
        assert_eq!(trades[0].user_order_id_buy, 111);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 52);
        assert_eq!(trades[1].price, 1.0);
        assert_eq!(trades[1].quantity, 2);
        assert_eq!(trades[1].user_id_buy, 10);
        assert_eq!(trades[1].user_order_id_buy, 110);
        assert_eq!(trades[1].user_id_sell, 2);
        assert_eq!(trades[1].user_order_id_sell, 52);

        // Remaining order are correct
        assert_eq!(book.asks.len(), 0);
        assert_eq!(book.bids.len(), 1);
        assert_eq!(book.bids.iter().next().unwrap().1.orders()[0].quantity(), 3);
    }
}
