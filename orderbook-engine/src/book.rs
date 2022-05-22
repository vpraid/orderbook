//! This module contains the definition of an order book - the primary structure for trading on the market.

use crate::level::Level;
use crate::order::Order;
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
    pub fn add(&mut self, _order: Order<'a>) {
        unimplemented!()
    }

    /// Cancel an given order, removing it from the order book immediately.
    pub fn remove(&mut self, _order: &Order<'a>) {
        unimplemented!()
    }

    /// Execute all trades in this order book.
    pub fn execute(&mut self) {
        unimplemented!()
    }
}

impl Default for Book<'_> {
    fn default() -> Self {
        Self::new()
    }
}
