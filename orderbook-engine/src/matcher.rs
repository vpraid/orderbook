//! This modules provides the implementation of order matching algorithms.
use crate::level::Level;
use crate::order::Order;
use crate::trade::Trade;

pub mod fifo;

pub trait Matcher {
    fn match_order<'a>(&mut self, order: &mut Order<'a>, level: &mut Level<'a>) -> Vec<Trade<'a>>;
}
