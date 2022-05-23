use super::*;

pub struct FIFOMatcher;

impl Matcher for FIFOMatcher {
    fn match_order(&mut self, order: &mut Order, level: &mut Level) -> Vec<Trade> {
        let mut trades = Vec::new();
        while let Some(other) = level.orders_mut().back_mut() {
            trades.push(order.match_to(other));
            if other.is_done() {
                level.orders_mut().pop_back();
            }
            if order.is_done() {
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
    use crate::{Price, Symbol};
    use string_interner::StringInterner;

    fn make_test_level(symbol: Symbol) -> Level {
        let orders = [
            Order::with_ids(1, 101).limit_order(Side::Ask, symbol, 1.0, 5),
            Order::with_ids(2, 102).limit_order(Side::Ask, symbol, 1.0, 10),
            Order::with_ids(3, 103).limit_order(Side::Ask, symbol, 1.0, 7),
        ];
        let mut level = Level::new(1.0.into(), Side::Ask);
        for order in orders {
            level.add(order);
        }
        level
    }

    #[test]
    fn test_very_small_order() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, aapl, 1.0, 2);
        let mut level = make_test_level(aapl);
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 2);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 1);
        assert_eq!(trades[0].user_order_id_sell, 101);

        // Remaning orders are correct
        assert_eq!(level.orders().len(), 3);
        assert_eq!(level.orders()[0].quantity(), 7);
        assert_eq!(level.orders()[1].quantity(), 10);
        assert_eq!(level.orders()[2].quantity(), 3);
    }

    #[test]
    fn test_small_order() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, aapl, 1.0, 6);
        let mut level = make_test_level(aapl);
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 1);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 1);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 2);
        assert_eq!(trades[1].user_order_id_sell, 102);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 2);
        assert_eq!(level.orders()[0].quantity(), 7);
        assert_eq!(level.orders()[1].quantity(), 9);
        assert!(bid_order.is_done());
    }

    #[test]
    fn test_medium_order() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, aapl, 1.0, 15);
        let mut level = make_test_level(aapl);
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 1);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 2);
        assert_eq!(trades[1].user_order_id_sell, 102);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 1);
        assert_eq!(level.orders()[0].quantity(), 7);
        assert!(bid_order.is_done());
    }

    #[test]
    fn test_large_order() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, aapl, 1.0, 18);
        let mut level = make_test_level(aapl);
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 1);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 2);
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
        assert!(bid_order.is_done());
    }

    #[test]
    fn test_very_large_order() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, aapl, 1.0, 22);
        let mut level = make_test_level(aapl);
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 1);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 2);
        assert_eq!(trades[1].user_order_id_sell, 102);
        assert_eq!(trades[2].price, Price::from(1.0));
        assert_eq!(trades[2].quantity, 7);
        assert_eq!(trades[2].user_id_buy, 4);
        assert_eq!(trades[2].user_order_id_buy, 51);
        assert_eq!(trades[2].user_id_sell, 3);
        assert_eq!(trades[2].user_order_id_sell, 103);

        // Remaining orders are correct
        assert_eq!(level.orders().len(), 0);
        assert!(bid_order.is_done());
    }

    #[test]
    fn test_gigantic_order() {
        let mut si = StringInterner::default();
        let aapl = si.get_or_intern_static("AAPL");
        let mut matcher = FIFOMatcher;
        let mut bid_order = Order::with_ids(4, 51).limit_order(Side::Bid, aapl, 1.0, 25);
        let mut level = make_test_level(aapl);
        let trades = matcher.match_order(&mut bid_order, &mut level);

        // Trades are correct
        assert_eq!(trades.len(), 3);
        assert_eq!(trades[0].price, Price::from(1.0));
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(trades[0].user_id_buy, 4);
        assert_eq!(trades[0].user_order_id_buy, 51);
        assert_eq!(trades[0].user_id_sell, 1);
        assert_eq!(trades[0].user_order_id_sell, 101);
        assert_eq!(trades[1].price, Price::from(1.0));
        assert_eq!(trades[1].quantity, 10);
        assert_eq!(trades[1].user_id_buy, 4);
        assert_eq!(trades[1].user_order_id_buy, 51);
        assert_eq!(trades[1].user_id_sell, 2);
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
