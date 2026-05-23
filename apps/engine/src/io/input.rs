use crate::types::{Side, Price, Qty, OrderType, OrderId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EngineInput {
    NewOrder {
        side: Side,
        price: Price,
        quantity: Qty,
        order_type: OrderType,
    },
    CancelOrder {
        order_id: OrderId,

    }
}

impl EngineInput {
    pub fn parse(line: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(line)
    }
}
