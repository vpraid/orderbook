use ordered_float::OrderedFloat;
use string_interner::symbol::SymbolU32;

mod book;
mod level;
mod market;
mod matcher;
mod order;
mod trade;

pub type Price = OrderedFloat<f64>;
pub type Symbol = SymbolU32;

pub mod prelude {
    pub use super::market::Market;
    pub use super::matcher::*;
    pub use super::order::{Order, Side};
    pub use super::trade::Trade;
    pub use super::Price;
    pub use super::Symbol;
}
