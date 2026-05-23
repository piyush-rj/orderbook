use crate::types::{OrderId, OrderStatus, Price, Qty, Side};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelOrderResult {
    pub order_id: OrderId,
    pub status: OrderStatus,
    pub remaining_quantity: Qty,
    pub side: Side,
    pub price: Price,
}

impl CancelOrderResult {
    pub fn new(
        order_id: OrderId,
        status: OrderStatus,
        remaining_quantity: Qty,
        side: Side,
        price: Price,
    ) -> Self {
        Self {
            order_id,
            status,
            remaining_quantity,
            side,
            price,
        }
    }
}
