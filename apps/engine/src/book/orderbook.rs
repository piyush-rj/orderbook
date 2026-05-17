use std::collections::{BTreeMap, VecDeque};
use crate::order::Order;
use crate::types::Price;

pub struct OrderBook {
    /* 150$, ["piyush", "rishi"]*/

    asks: BTreeMap<Price, VecDeque<Order>>, 
    bids: BTreeMap<Price, VecDeque<Order>>,
    next_order_id: u64,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            next_order_id: 1,
        }
    }
}
