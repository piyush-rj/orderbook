use crate::types::{OrderId, Price, Qty, Side, TradeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub trade_id: TradeId,
    pub maker_order_id: OrderId,
    pub taker_order_id: OrderId,
    pub price: Price,
    pub quantity: Qty,
    pub side: Side,
}

impl Trade {
    pub fn new(
        trade_id: TradeId,
        maker_order_id: OrderId,
        taker_order_id: OrderId,
        price: Price,
        quantity: Qty,
        side: Side,
    ) -> Self {
        Self {
            trade_id,
            maker_order_id,
            taker_order_id,
            price,
            quantity,
            side,
        }
    }
}
