use orderbook_common::{CancelOrder, Command, NewOrder};
use orderbook_engine::prelude::*;

use string_interner::StringInterner;
use tokio::sync::mpsc;

pub async fn run(mut rx: mpsc::Receiver<Command>) {
    let mut si = StringInterner::default();
    let mut market = Market::new(FIFOMatcher);
    while let Some(command) = rx.recv().await {
        match command {
            Command::New(order) => {
                let order = limit_order(order, &mut si);
                market.add(order);
            }
            Command::Cancel(CancelOrder {
                user_id,
                user_order_id,
            }) => {
                market.cancel(user_id, user_order_id);
            }
            Command::Flush => {
                market.clear();
            }
        }
    }
}

fn limit_order(order: NewOrder, si: &mut StringInterner) -> Order {
    let side = decode_side(order.side);
    Order::with_ids(order.user_id, order.user_order_id).limit_order(
        side,
        si.get_or_intern(order.symbol),
        order.price as f64,
        order.quantity,
    )
}

fn decode_side(side: char) -> Side {
    match side {
        'B' => Side::Bid,
        'S' => Side::Ask,
        _ => panic!("Invalid side"),
    }
}
