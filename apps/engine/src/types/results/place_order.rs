use crate::types::{OrderId, OrderStatus, Qty, Trade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderResult {
    pub order_id: OrderId,
    pub trades: Vec<Trade>,
    pub status: OrderStatus,
    pub filled_quantity: Qty,
    pub remaining_quantity: Qty,
}

impl PlaceOrderResult {
    pub fn new(
        order_id: OrderId,
        trades: Vec<Trade>,
        status: OrderStatus,
        filled_quantity: Qty,
        remaining_quantity: Qty,
    ) -> Self {
        Self {
            order_id,
            trades,
            status,
            filled_quantity,
            remaining_quantity,
        }
    }

    pub fn rejected(order_id: OrderId, remaining_quantity: Qty) -> Self {
        Self {
            order_id,
            trades: Vec::new(),
            status: OrderStatus::Rejected,
            filled_quantity: 0,
            remaining_quantity,
        }
    }
}
