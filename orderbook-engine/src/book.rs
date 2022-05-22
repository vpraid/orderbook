//! This module contains the definition of an order book - the primary structure for trading on the market.

use crate::level::Level;
use crate::market::Index;
use crate::order::{Order, Side};
use crate::trade::Trade;
use crate::Price;

use std::cmp::Reverse;
use std::collections::BTreeMap;

/// Book represents an order book. It is implemented as a collection of levels for bid and for ask
/// orders separately. When a match must be done, the top level from the bids collection and the
/// bottom level from the asks collection are matched according to a matching algorithm.
#[allow(dead_code)]
pub struct Book<'a> {
    bids: BTreeMap<Price, Level<'a>>,
    asks: BTreeMap<Reverse<Price>, Level<'a>>,
}

#[allow(dead_code)]
impl<'a> Book<'a> {
    /// Create a new order book.
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    /// Add an order to the book.
    pub fn add(&mut self, order: Order<'a>) {
        let price = order.price();
        let level = match order.side() {
            Side::Bid => self.bids.entry(price).or_insert_with(|| Level::new(price)),
            Side::Ask => self
                .asks
                .entry(Reverse(price))
                .or_insert_with(|| Level::new(price)),
        };
        level.add(order);
    }

    /// Cancel an given order, removing it from the order book immediately.
    pub fn remove(&mut self, index: &Index<'a>) -> Option<Order<'a>> {
        let level = match index.side {
            Side::Bid => self.bids.get_mut(&index.price)?,
            Side::Ask => self.asks.get_mut(&Reverse(index.price))?,
        };
        level.remove(index.user_id, index.user_order_id)
    }

    /// Execute all trades in this order book.
    pub fn execute(&mut self) -> Vec<Trade<'a>> {
        unimplemented!()
    }
}

impl Default for Book<'_> {
    fn default() -> Self {
        Self::new()
    }
}
