use crate::types::{OrderId, OrderType, Price, Qty, Side};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub price: Price,
    pub quantity: Qty,
    pub order_type: OrderType,
    pub remaining: Qty,
}

impl Order {
    pub fn new(
        id: OrderId,
        side: Side,
        price: Price,
        quantity: Qty,
        order_type: OrderType,
        remaining: Qty,
    ) -> Self {
        Self {
            id,
            side,
            price,
            quantity,
            order_type,
            remaining,
        }
    }
}
