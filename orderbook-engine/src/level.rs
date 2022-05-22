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
