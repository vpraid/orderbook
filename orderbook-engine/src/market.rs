//! This modulev contains the definition of a market. A market is a collection of order books
//! for a given set of securities.

use crate::book::Book;
use crate::order::Order;
use crate::trade::Trade;

use std::collections::HashMap;

/// Market is a collection of order books for a given set of securities. It also contains a map
/// of all orders currently on the market for easy lookup.
#[allow(dead_code)]
pub struct Market<'a> {
    books: HashMap<&'a str, Book<'a>>,
    orders: HashMap<(u64, u64), Order<'a>>,
}

impl<'a> Market<'a> {
    /// Create a new market.
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            orders: HashMap::new(),
        }
    }

    /// Add an order to the market.
    pub fn add(&mut self, _order: Order<'a>) {
        unimplemented!()
    }

    /// Cancel an order given by order ids.
    pub fn cancel(&mut self, _user_id: u64, _user_order_id: u64) {
        unimplemented!()
    }

    /// Execute trades on the market, updating the order books and removing orders that were
    /// successfully traded.
    pub fn execute(&mut self) -> Vec<Trade<'a>> {
        unimplemented!()
    }
}

impl Default for Market<'_> {
    fn default() -> Self {
        Self::new()
    }
}
