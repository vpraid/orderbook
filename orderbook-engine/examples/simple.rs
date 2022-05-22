use orderbook_engine::prelude::*;

fn main() {
    let orders = vec![
        Order::with_ids(1, 101).limit_order(Side::Bid, "AAPL", 100.0, 15),
        Order::with_ids(2, 102).limit_order(Side::Ask, "GOOG", 50.0, 20),
        Order::with_ids(2, 103).limit_order(Side::Ask, "AAPL", 50.0, 14),
    ];

    let mut market = Market::new(FIFOMatcher);
    let trades = orders
        .into_iter()
        .flat_map(|order| market.add(order))
        .collect::<Vec<_>>();

    println!("Traded securities: {:?}", trades);
}
