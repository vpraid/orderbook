use ordered_float::OrderedFloat;

mod order;

pub type Price = OrderedFloat<f64>;

pub mod prelude {
    pub use super::order::*;
}
