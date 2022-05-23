//! The module contains the definition of a price level in an order book.
use crate::order::{Order, Side};
use crate::Price;

use std::collections::VecDeque;

/// Level represents a price level in an order book. The orders in a level are
/// placed in a deque for efficient processing by matching algorithms.
pub struct Level {
    price: Price,
    side: Side,
    orders: VecDeque<Order>,
}

#[allow(dead_code)]
impl<'a> Level {
    /// Create a new price level.
    pub fn new(price: Price, side: Side) -> Self {
        Self {
            price,
            side,
            orders: VecDeque::new(),
        }
    }

    /// Add an order to the level.
    pub fn add(&mut self, order: Order) {
        debug_assert!(
            order.side() == self.side,
            "Order side does not match level side"
        );
        self.orders.push_front(order);
    }

    /// Cancel an order given by order ids.
    pub fn remove(&mut self, user_id: u64, user_order_id: u64) -> Option<Order> {
        for (idx, order) in self.orders.iter().enumerate() {
            if order.user_id() == user_id && order.user_order_id() == user_order_id {
                return self.orders.remove(idx);
            }
        }
        None
    }

    /// Get the price of this level.
    pub fn price(&self) -> Price {
        self.price
    }

    pub fn orders(&self) -> &VecDeque<Order> {
        &self.orders
    }

    pub fn orders_mut(&mut self) -> &mut VecDeque<Order> {
        &mut self.orders
    }

    /// Return true if this level doesn't contain any orders.
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }
}
