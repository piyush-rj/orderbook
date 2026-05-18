use std::collections::{BTreeMap, VecDeque};
use crate::order::Order;
use crate::types::Price;
use crate::types::Side;

pub struct OrderBook {
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

     pub fn add_as_resting_order(&mut self, order: Order) {
        let side = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks
        };

        if side.contains_key(&order.price) {
            let queue = side.get_mut(&order.price).unwrap();
            queue.push_back(order);
        } else {
            let price = order.price;
            let mut queue = VecDeque::new();
            queue.push_back(order);
            side.insert(price, queue);
        }
    }

     pub fn get_best_ask(&self) {
     }

     pub fn get_best_bid(&self) {
     }
    

}
