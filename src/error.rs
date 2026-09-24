#[derive(Debug)]
pub enum AccountingError{
    Overflow,
    DivisionByZero,
    InsufficientShares,
}
