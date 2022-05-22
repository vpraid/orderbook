//! This modules provides the implementation of order matching algorithms.
use crate::level::Level;
use crate::order::Order;
use crate::trade::Trade;

pub trait Matcher {
    fn match_order<'a>(&mut self, order: &mut Order<'a>, level: &mut Level<'a>) -> Vec<Trade<'a>>;
}

struct FIFOMatcher;

impl Matcher for FIFOMatcher {
    fn match_order<'a>(&mut self, order: &mut Order<'a>, level: &mut Level<'a>) -> Vec<Trade<'a>> {
        debug_assert!(
            order.price() == level.price(),
            "Order price does not match level price"
        );
        let mut trades = Vec::new();
        while let Some(mut other) = level.orders_mut().pop_back() {
            let trade = order.trade(&mut other);
            trades.push(trade);
            // Resting orders must reenter the level from the front like new orders
            if other.is_valid() {
                level.add(other);
            }
            if !order.is_valid() {
                break;
            }
        }
        trades
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::Level;
    use crate::order::{Order, Side};
    use crate::Price;

    fn make_test_level() -> Level<'static> {
        let orders = [
            Order::with_ids(2, 101).limit_order(Side::Ask, "AAPL", 1.0, 5),
            Order::with_ids(1, 102).limit_order(Side::Ask, "AAPL", 1.0, 10),
            Order::with_ids(3, 103).limit_order(Side::Ask, "AAPL", 1.0, 7),
        ];
        let mut level = Level::new(1.0.into());
        for order in orders {
            level.add(order);
        }
        level
    }

    #[test]
    fn test_very_small_order() {
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, "AAPL", 1.0, 2);
        let mut level = make_test_level();
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 2);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 101);

        // Remaning orders are correct
        assert_eq!(level.orders().len(), 3);
        assert_eq!(level.orders()[0].quantity(), 3);
        assert_eq!(level.orders()[1].quantity(), 7);
        assert_eq!(level.orders()[2].quantity(), 10);
    }

    #[test]
    fn test_small_order() {
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, "AAPL", 1.0, 6);
        let mut level = make_test_level();
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 1);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 1);
        assert_eq!(trades[1].user_order_id_sell, 102);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 2);
        assert_eq!(level.orders()[0].quantity(), 9);
        assert_eq!(level.orders()[1].quantity(), 7);
        assert!(!bid_order.is_valid());
    }

    #[test]
    fn test_medium_order() {
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, "AAPL", 1.0, 15);
        let mut level = make_test_level();
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 1);
        assert_eq!(trades[1].user_order_id_sell, 102);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 1);
        assert_eq!(level.orders()[0].quantity(), 7);
        assert!(!bid_order.is_valid());
    }

    #[test]
    fn test_large_order() {
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, "AAPL", 1.0, 18);
        let mut level = make_test_level();
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 1);
        assert_eq!(trades[1].user_order_id_sell, 102);
        assert_eq!(trades[2].price, Price::from(1.0));
        assert_eq!(trades[2].quantity, 3);
        assert_eq!(trades[2].user_id_buy, 4);
        assert_eq!(trades[2].user_order_id_buy, 51);
        assert_eq!(trades[2].user_id_sell, 3);
        assert_eq!(trades[2].user_order_id_sell, 103);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 1);
        assert_eq!(level.orders()[0].quantity(), 4);
        assert!(!bid_order.is_valid());
    }

    #[test]
    fn test_very_large_order() {
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, "AAPL", 1.0, 22);
        let mut level = make_test_level();
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 1);
        assert_eq!(trades[1].user_order_id_sell, 102);
        assert_eq!(trades[2].price, Price::from(1.0));
        assert_eq!(trades[2].quantity, 7);
        assert_eq!(trades[2].user_id_buy, 4);
        assert_eq!(trades[2].user_order_id_buy, 51);
        assert_eq!(trades[2].user_id_sell, 3);
        assert_eq!(trades[2].user_order_id_sell, 103);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 0);
        assert!(!bid_order.is_valid());
    }

    #[test]
    fn test_gigantic_order() {
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, "AAPL", 1.0, 25);
        let mut level = make_test_level();
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 2);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 1);
        assert_eq!(trades[1].user_order_id_sell, 102);
        assert_eq!(trades[2].price, Price::from(1.0));
        assert_eq!(trades[2].quantity, 7);
        assert_eq!(trades[2].user_id_buy, 4);
        assert_eq!(trades[2].user_order_id_buy, 51);
        assert_eq!(trades[2].user_id_sell, 3);
        assert_eq!(trades[2].user_order_id_sell, 103);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 0);
        assert_eq!(bid_order.quantity(), 3);
    }
}
