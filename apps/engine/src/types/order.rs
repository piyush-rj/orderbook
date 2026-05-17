use super::{OrderId, Price, Qty, Side};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub price: Price,
    pub quantity: Qty,
    pub remaining: Qty,
}
