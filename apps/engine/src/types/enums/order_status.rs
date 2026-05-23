#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    Open,
    Filled,
    PartiallyFilled,
    Cancelled,
    Rejected,
}
