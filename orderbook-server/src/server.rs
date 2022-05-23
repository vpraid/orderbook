use std::sync::{Arc, Mutex};

use orderbook_common::{CancelOrder, Command, NewOrder};
use orderbook_engine::prelude::*;

use string_interner::StringInterner;
use tokio::sync::mpsc;

// Using mutex for synchornizing access to the orderbook in HFT is a bad idea. Ideally,
// the markey, the orderbooks and all internal datastructures should be made lock-free.
// It is possible, e.g. to use a lock-free skip list instead of a BTreeMap for price
// levels, but rust ecosystems lacks a stable crate with a good api for that purpose.
pub async fn run(mut rx: mpsc::Receiver<Command>) {
    let mut si = StringInterner::default();
    let market = Arc::new(Mutex::new(Market::new(FIFOMatcher)));
    while let Some(command) = rx.recv().await {
        match command {
            Command::New(order) => {
                let order = limit_order(order, &mut si);
                market.lock().unwrap().add(order);
            }
            Command::Cancel(CancelOrder {
                user_id,
                user_order_id,
            }) => {
                market.lock().unwrap().cancel(user_id, user_order_id);
            }
            Command::Flush => {
                market.lock().unwrap().clear();
                println!();
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
