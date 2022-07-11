use crate::types::NumberType;
use bigdecimal::{BigDecimal, FromPrimitive};
use num::bigint::{BigInt, ToBigInt};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum PrecisionNum {
    Int(BigInt),
    Float(BigDecimal),
    Rational(BigInt, BigInt),
}

impl NumberType for PrecisionNum {
    fn zero() -> PrecisionNum {
        PrecisionNum::Int(0.to_bigint().unwrap())
    }

    fn one() -> PrecisionNum {
        PrecisionNum::Int(1.to_bigint().unwrap())
    }

    fn try_create_rational(num: Self, den: Self) -> Self {
        match (num, den) {
            (PrecisionNum::Int(v1), PrecisionNum::Int(v2)) => PrecisionNum::Rational(v1, v2),
            _ => {
                panic!("Creating rational from non integer not yet implemented.");
            }
        }
    }

    fn shorten(&mut self) {}

    fn pow(&self, _exp: Self) -> Self {
        panic!("Implementierung notwendig");
    }

    fn abs(&self) -> Self {
        panic!("Implementierung notwendig");
    }

    fn is_integer(&self) -> bool {
        match self {
            PrecisionNum::Int(_) => true,
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match self {
            PrecisionNum::Float(_) => true,
            _ => false,
        }
    }

    fn is_rational(&self) -> bool {
        match self {
            PrecisionNum::Rational(_, _) => true,
            _ => false,
        }
    }

    fn get_numerator(&self) -> Self {
        panic!("Implementierung notwendig");
    }

    fn get_denominator(&self) -> Self {
        panic!("Implementierung notwendig");
    }
}

impl Into<i128> for PrecisionNum {
    fn into(self) -> i128 {
        panic!("Implementierung notwendig");
    }
}

impl From<i128> for PrecisionNum {
    fn from(from: i128) -> PrecisionNum {
        PrecisionNum::Int(from.to_bigint().unwrap())
    }
}

impl From<f64> for PrecisionNum {
    fn from(from: f64) -> PrecisionNum {
        PrecisionNum::Float(BigDecimal::from_f64(from).unwrap())
    }
}

impl ops::Add<PrecisionNum> for PrecisionNum {
    type Output = PrecisionNum;

    fn add(self, rhs: PrecisionNum) -> PrecisionNum {
        match (self, rhs) {
            (PrecisionNum::Int(v1), PrecisionNum::Int(v2)) => PrecisionNum::Int(v1 + v2),
            (PrecisionNum::Float(v1), PrecisionNum::Float(v2)) => PrecisionNum::Float(v1 + v2),
            _ => {
                panic!("Implementierung notwendig");
            }
        }
    }
}

impl ops::Mul<PrecisionNum> for PrecisionNum {
    type Output = PrecisionNum;

    fn mul(self, _rhs: PrecisionNum) -> PrecisionNum {
        panic!("Implementierung notwendig");
    }
}

impl ops::Rem<i128> for PrecisionNum {
    type Output = PrecisionNum;

    fn rem(self, _rhs: i128) -> PrecisionNum {
        panic!("Implementierung notwendig");
    }
}

impl PartialEq<i128> for PrecisionNum {
    fn eq(&self, other: &i128) -> bool {
        match self {
            PrecisionNum::Int(v) => *v == other.to_bigint().unwrap().clone(),
            PrecisionNum::Float(v) => *v == BigDecimal::from_i128(*other).unwrap(),
            _ => false,
        }
    }
}

impl PartialOrd<i128> for PrecisionNum {
    fn partial_cmp(&self, _other: &i128) -> Option<Ordering> {
        panic!("Implementierung notwendig");
    }

    fn lt(&self, _other: &i128) -> bool {
        panic!("Implementierung notwendig");
    }

    fn le(&self, _other: &i128) -> bool {
        panic!("Implementierung notwendig");
    }

    fn gt(&self, _other: &i128) -> bool {
        panic!("Implementierung notwendig");
    }

    fn ge(&self, _other: &i128) -> bool {
        panic!("Implementierung notwendig");
    }
}

impl Eq for PrecisionNum {}

impl Ord for PrecisionNum {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PrecisionNum::Int(_), PrecisionNum::Float(_))
            | (PrecisionNum::Int(_), PrecisionNum::Rational(_, _))
            | (PrecisionNum::Float(_), PrecisionNum::Rational(_, _)) => Ordering::Less,
            (PrecisionNum::Float(_), PrecisionNum::Int(_))
            | (PrecisionNum::Rational(_, _), PrecisionNum::Int(_))
            | (PrecisionNum::Rational(_, _), PrecisionNum::Float(_)) => Ordering::Greater,
            (PrecisionNum::Int(v1), PrecisionNum::Int(v2)) => v1.cmp(v2),
            (PrecisionNum::Float(v1), PrecisionNum::Float(v2)) => {
                if v1 > v2 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (PrecisionNum::Rational(num1, den1), PrecisionNum::Rational(num2, den2)) => {
                if num1 / den1 > num2 / den2 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

impl Hash for PrecisionNum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PrecisionNum::Int(v) => {
                v.hash(state);
            }
            PrecisionNum::Float(v) => {
                v.to_string().hash(state);
            }
            PrecisionNum::Rational(num, den) => {
                num.hash(state);
                den.hash(state);
            }
        }
    }
}

impl std::fmt::Display for PrecisionNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PrecisionNum::Int(v) => {
                write!(f, "{}", v)
            }
            PrecisionNum::Float(v) => {
                write!(f, "{}", v)
            }
            PrecisionNum::Rational(num, den) => {
                write!(f, "({}/{})", num, den)
            }
        }
    }
}
