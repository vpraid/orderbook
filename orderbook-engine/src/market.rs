//! This modulev contains the definition of a market. A market is a collection of order books
//! for a given set of securities.

use crate::book::Book;
use crate::matcher::Matcher;
use crate::order::{Order, Side};
use crate::trade::Trade;
use crate::Price;

use std::collections::HashMap;

/// Helper structure to tracker orders by their ids. This is necessary when we want for example
/// to cancel an order, but know only its ids. Since we need the symbol to choose a correct order
/// book and a price to choose a price level in the book to cancnel an order, we keep track of
/// this information with this structure.
#[allow(dead_code)]
pub struct Index<'a> {
    pub user_id: u64,
    pub user_order_id: u64,
    pub symbol: &'a str,
    pub price: Price,
    pub side: Side,
}

impl<'a> Index<'a> {
    fn from_order(order: &Order<'a>) -> Self {
        Self {
            user_id: order.user_id(),
            user_order_id: order.user_order_id(),
            symbol: order.symbol(),
            price: order.price(),
            side: order.side(),
        }
    }

    pub fn ids(&self) -> (u64, u64) {
        (self.user_id, self.user_order_id)
    }
}

/// Market is a collection of order books for a given set of securities. It also contains a map
/// of all index structs for all orders currently on the market.
#[allow(dead_code)]
pub struct Market<'a, M> {
    books: HashMap<&'a str, Book<'a>>,
    indices: HashMap<(u64, u64), Index<'a>>,
    matcher: M,
}

impl<'a, M: Matcher> Market<'a, M> {
    /// Create a new market.
    pub fn new(matcher: M) -> Self {
        Self {
            books: HashMap::new(),
            indices: HashMap::new(),
            matcher,
        }
    }

    /// Add an order to the market.
    pub fn add(&mut self, order: Order<'a>) {
        let index = Index::from_order(&order);
        self.books.entry(order.symbol()).or_default().add(order);
        self.indices.insert(index.ids(), index);
    }

    /// Cancel an order given by order ids.
    pub fn cancel(&mut self, user_id: u64, user_order_id: u64) -> Option<Order<'a>> {
        // Find the index of the order to cancel, find the book and remove
        // the order from the book.
        let index = self
            .indices
            .get(&(user_id, user_order_id))
            .expect("Index not found");
        let book = self.books.get_mut(index.symbol).expect("Book not found");
        let removed_order = book.remove(index);
        // We don't want to remove a book when its empty. It is an unsual situation
        // to have no orders for a specific security at all in the first place, and
        // even if it happens, we probably will have a new order for it soon. Therefore,
        // it is enough to update indices only.
        let ids = index.ids();
        self.indices.remove(&ids).expect("Index not found");
        removed_order
    }

    /// Execute trades on the market, updating the order books and removing orders that were
    /// successfully traded.
    pub fn execute(&mut self) -> Vec<Trade<'a>> {
        let matcher = &mut self.matcher;
        self.books
            .values_mut()
            .flat_map(|book| book.execute(matcher))
            .collect()
    }
}
