use bigdecimal::BigDecimal;
use num::bigint::BigInt;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum PrecisionNum {
    Int(BigInt),
    Float(BigDecimal),
    Rational(BigInt, BigInt),
}
