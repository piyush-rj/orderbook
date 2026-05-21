#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
    IOC,
    FOK,
}
