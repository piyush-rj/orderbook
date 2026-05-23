use std::collections::HashMap;
use std::collections::{BTreeMap, VecDeque};
use crate::{CancelOrderResult, Order, OrderId, OrderStatus, OrderType, PlaceOrderResult};
use crate::Qty;
use crate::Trade;
use crate::types::Price;
use crate::types::Side;

pub struct OrderBook {
    asks: BTreeMap<Price, VecDeque<Order>>, 
    bids: BTreeMap<Price, VecDeque<Order>>,
    next_order_id: u64,
    next_trade_id: u64,
    order_map: HashMap<OrderId, (Price, Side)>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            next_order_id: 1,
            next_trade_id: 1,
            order_map: HashMap::new(),
        }
    }

    pub fn place_order(&mut self, side: Side, price: Price, quantity: Qty, order_type: OrderType) -> PlaceOrderResult {
        if quantity == 0 {
            eprintln!("[book] REJECT zero-qty order");
            return PlaceOrderResult::rejected(self.next_order_id, quantity);
        }

        let mut taker_order = Order::new(self.next_order_id, side, price, quantity, order_type, quantity);
        self.next_order_id += 1;

        eprintln!(
            "[book] taker #{} {:?} {:?} qty={} @ {}",
            taker_order.id, taker_order.order_type, taker_order.side, taker_order.quantity, taker_order.price
        );

        let mut trades: Vec<Trade> = Vec::new();

        while taker_order.remaining > 0 {
            
            let side: Side = taker_order.side;

            let best_price = match side {
                Side::Buy => self.get_best_ask(),
                Side::Sell => self.get_best_bid(),
            };

            let best_price = match best_price {
                Some(p) => p,
                None => break,
            };

            let trade_executable = match taker_order.order_type {
                OrderType::Market => true,
                OrderType::Limit => match side {
                    Side::Buy => taker_order.price >= best_price,
                    Side::Sell => taker_order.price <= best_price,
                },
            };

            if !trade_executable {
                break;
            }

            let maker_queue = match side {
                Side::Buy => self.asks.get_mut(&best_price).unwrap(),
                Side::Sell => self.bids.get_mut(&best_price).unwrap(),
            };

            let maker = maker_queue.front_mut().unwrap();
            let match_quantity = taker_order.remaining.min(maker.remaining);

            let trade = Trade::new(self.next_trade_id, maker.id, taker_order.id, best_price, match_quantity, side);
            eprintln!(
                "[book] MATCH taker #{} <-> maker #{} qty={} @ {}",
                taker_order.id, maker.id, match_quantity, best_price
            );
            trades.push(trade);

            self.next_trade_id += 1;

            taker_order.remaining -= match_quantity;
            maker.remaining -= match_quantity;

            if maker.remaining == 0 {
                let maker_id = maker.id;
                maker_queue.pop_front();
                self.order_map.remove(&maker_id);
            }

            let is_queue_empty = maker_queue.is_empty();
            if is_queue_empty {
                match side {
                    Side::Buy => self.asks.remove(&best_price),
                    Side::Sell => self.bids.remove(&best_price),
                };
            }

            if taker_order.remaining == 0 {
                break;
            }

        }

        let taker_id = taker_order.id;
        let taker_remaining_qty = taker_order.remaining;
        let filled_qty = taker_order.quantity - taker_remaining_qty;
        let taker_order_type = taker_order.order_type;

        if taker_remaining_qty > 0 {
            match taker_order_type {
                OrderType::Limit => {
                    eprintln!("[book] REST #{} qty={} @ {}", taker_id, taker_remaining_qty, price);
                    self.add_as_resting_order(taker_order);
                }
                OrderType::Market => {
                    eprintln!("[book] DROP #{} qty={} (market leftover)", taker_id, taker_remaining_qty);
                }
            }
        }

        let status = if taker_remaining_qty == 0 {
            OrderStatus::Filled
        } else if filled_qty > 0 {
            match taker_order_type {
                OrderType::Limit => OrderStatus::PartiallyFilled,
                OrderType::Market => OrderStatus::Cancelled,
            }
        } else {
            match taker_order_type {
                OrderType::Limit => OrderStatus::Open,
                OrderType::Market => OrderStatus::Cancelled,
            }
        };

        PlaceOrderResult::new(taker_id, trades, status, filled_qty, taker_remaining_qty)
    }

    pub fn cancel_order(&mut self, order_id: OrderId) -> CancelOrderResult {
        let (price, side) = match self.order_map.get(&order_id) {
            Some(&(p, s)) => (p, s),
            None => {
                eprintln!("[book] CANCEL REJECT #{} not found", order_id);
                return CancelOrderResult::new(order_id, OrderStatus::Rejected, 0, Side::Buy, 0);
            }
        };

        let queue = match side {
            Side::Buy => self.bids.get_mut(&price).unwrap(),
            Side::Sell => self.asks.get_mut(&price).unwrap(),
        };

        let order_pos = queue.iter().position(|o| o.id == order_id).unwrap();
        let removed = queue.remove(order_pos).unwrap();

        if queue.is_empty() {
            match side {
                Side::Buy => self.bids.remove(&price),
                Side::Sell => self.asks.remove(&price),
            };
        }

        self.order_map.remove(&order_id);
        eprintln!(
            "[book] CANCEL #{} {:?} qty={} @ {}",
            order_id, side, removed.remaining, price
        );
        CancelOrderResult::new(order_id, OrderStatus::Cancelled, removed.remaining, side, price)
    }

    pub fn add_as_resting_order(&mut self, order: Order) {
        self.order_map.insert(order.id, (order.price, order.side));

        let side = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
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

    pub fn get_best_ask(&self) -> Option<Price> {
        if let Some((price, _)) = self.asks.first_key_value(){
            Some(*price)
        } else {
            None
        } 
    }

    pub fn get_best_bid(&self) -> Option<Price> {
        if let Some((price, _)) = self.bids.last_key_value() {
            Some(*price)
        } else {
            None
        }
    }

}


