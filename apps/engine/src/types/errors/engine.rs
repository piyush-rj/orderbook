use crate::types::OrderId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    OrderNotFound(OrderId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOrderError {
    InvalidQuantity,
    InvalidPrice,
    MarketUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelOrderError {
    NotFound(OrderId),
}
