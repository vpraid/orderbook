use orderbook_engine::prelude::*;
use string_interner::StringInterner;

fn main() {
    let mut si = StringInterner::default();
    let orders = vec![
        Order::with_ids(1, 101).limit_order(Side::Bid, si.get_or_intern_static("AAPL"), 100.0, 15),
        Order::with_ids(2, 102).limit_order(Side::Ask, si.get_or_intern_static("GOOG"), 50.0, 20),
        Order::with_ids(2, 103).limit_order(Side::Ask, si.get_or_intern_static("AAPL"), 50.0, 14),
    ];

    let mut market = Market::new(FIFOMatcher);
    let trades = orders
        .into_iter()
        .flat_map(|order| market.add(order))
        .collect::<Vec<_>>();

    println!("Traded securities: {:?}", trades);
}
