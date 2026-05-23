use crate::book::OrderBook;
use crate::types::{
    CancelOrderResult, OrderId, OrderType, PlaceOrderResult, Price, Qty, Side,
};

pub struct Engine {
    book: OrderBook,
}

impl Engine {
    pub fn new() -> Self {
        Self { book: OrderBook::new() }
    }

    pub fn place_order(
        &mut self,
        side: Side,
        price: Price,
        quantity: Qty,
        order_type: OrderType,
    ) -> PlaceOrderResult {
        self.book.place_order(side, price, quantity, order_type)
    }

    pub fn cancel_order(
        &mut self,
        order_id: OrderId,
    ) -> CancelOrderResult {
        self.book.cancel_order(order_id)
    }
}
