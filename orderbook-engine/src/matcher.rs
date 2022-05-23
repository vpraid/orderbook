//! This modules provides the implementation of order matching algorithms.
use crate::level::Level;
use crate::order::Order;
use crate::trade::Trade;

pub mod fifo;
pub use fifo::FIFOMatcher;

pub trait Matcher {
    fn match_order(&mut self, order: &mut Order, level: &mut Level) -> Vec<Trade>;
}
