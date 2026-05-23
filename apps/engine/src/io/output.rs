use crate::types::{PlaceOrderResult, CancelOrderResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineOutput {
    PlaceResult(PlaceOrderResult),
    CancelResult(CancelOrderResult),
}

impl EngineOutput {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
