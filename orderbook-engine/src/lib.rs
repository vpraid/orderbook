use ordered_float::OrderedFloat;

mod book;
mod level;
mod market;
mod matcher;
mod order;
mod trade;

pub type Price = OrderedFloat<f64>;

pub mod prelude {
    pub use super::market::Market;
    pub use super::order::{Order, Side};
    pub use super::trade::Trade;
}
