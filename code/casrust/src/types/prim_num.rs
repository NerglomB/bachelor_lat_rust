use crate::types::NumberType;
use gcd::Gcd;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum PrimNum {
    Int(i128),
    Float(f64),
    Rational(i128, i128),
}

impl NumberType for PrimNum {
    fn zero() -> PrimNum {
        PrimNum::Int(0)
    }

    fn one() -> PrimNum {
        PrimNum::Int(1)
    }

    // Vll ummodeln in Error-Typ? verbesserung
    fn try_create_rational(num: Self, den: Self) -> Self {
        match (num, den) {
            (PrimNum::Int(v1), PrimNum::Int(v2)) => PrimNum::Rational(v1, v2),
            _ => {
                panic!("Creating rational from non integer not yet implemented.");
            }
        }
    }

    fn shorten(&mut self) {
        match self {
            PrimNum::Rational(num, den) => {
                if num == den {
                    *self = PrimNum::Int(1);
                } else if *num % *den == 0 {
                    *self = PrimNum::Int(*num / *den)
                } else if *num > 0 {
                    let gcd_v = (*num as u128).gcd(*den as u128) as i128;
                    *self = PrimNum::Rational(*num / gcd_v, *den / gcd_v)
                }
            }
            _ => {}
        }
    }

    fn pow(&self, exp: Self) -> Self {
        match (self, exp) {
            (PrimNum::Int(v1), PrimNum::Int(v2)) => PrimNum::Int(v1.pow(v2 as u32)),
            (PrimNum::Float(v1), PrimNum::Float(v2)) => PrimNum::Float(v1.powf(v2)),
            (PrimNum::Rational(n1, d1), PrimNum::Rational(n2, d2)) => {
                PrimNum::Float((*n1 as f64 / *d1 as f64).powf(n2 as f64 / d2 as f64))
            }
            (PrimNum::Int(v1), PrimNum::Float(v2)) => PrimNum::Float((*v1 as f64).powf(v2)),
            (PrimNum::Float(v1), PrimNum::Int(v2)) => PrimNum::Float(v1.powf(v2 as f64)),
            (PrimNum::Int(v1), PrimNum::Rational(num, den)) => {
                PrimNum::Float((*v1 as f64).powf(num as f64 / den as f64))
            }
            (PrimNum::Rational(num, den), PrimNum::Int(v2)) => {
                PrimNum::Float((*num as f64 / *den as f64).powf(v2 as f64))
            }
            (PrimNum::Float(v1), PrimNum::Rational(num, den)) => {
                PrimNum::Float(v1.powf(num as f64 / den as f64))
            }
            (PrimNum::Rational(num, den), PrimNum::Float(v2)) => {
                PrimNum::Float((*num as f64 / *den as f64).powf(v2))
            }
        }
    }

    fn abs(&self) -> Self {
        match self {
            PrimNum::Int(v) => PrimNum::Int(v.abs()),
            PrimNum::Float(v) => PrimNum::Float(v.abs()),
            PrimNum::Rational(num, den) => PrimNum::Rational(num.abs(), den.abs()),
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            PrimNum::Int(_) => true,
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match self {
            PrimNum::Float(_) => true,
            _ => false,
        }
    }

    fn is_rational(&self) -> bool {
        match self {
            PrimNum::Rational(_, _) => true,
            _ => false,
        }
    }

    fn get_numerator(&self) -> Self {
        match self {
            PrimNum::Int(v) => PrimNum::Int(*v),
            PrimNum::Float(v) => PrimNum::Float(*v),
            PrimNum::Rational(num, _) => PrimNum::Int(*num),
        }
    }

    fn get_denominator(&self) -> Self {
        match self {
            PrimNum::Int(_) | PrimNum::Float(_) => PrimNum::Int(1),
            PrimNum::Rational(_, den) => PrimNum::Int(*den),
        }
    }
}

impl Into<i128> for PrimNum {
    fn into(self) -> i128 {
        match self {
            PrimNum::Int(v) => v,
            _ => {
                panic!("PrimNum to i128 for float and rational not implemented");
            }
        }
    }
}

impl From<i128> for PrimNum {
    fn from(from: i128) -> PrimNum {
        PrimNum::Int(from)
    }
}

impl From<f64> for PrimNum {
    fn from(from: f64) -> PrimNum {
        PrimNum::Float(from)
    }
}

impl ops::Add<PrimNum> for PrimNum {
    type Output = PrimNum;

    fn add(self, rhs: PrimNum) -> PrimNum {
        match (self, rhs) {
            (PrimNum::Int(v1), PrimNum::Int(v2)) => PrimNum::Int(v1 + v2),
            (PrimNum::Float(v1), PrimNum::Float(v2)) => PrimNum::Float(v1 + v2),
            (PrimNum::Rational(num1, den1), PrimNum::Rational(num2, den2)) => {
                let mut t = PrimNum::Rational(num1 * den2 + num2 * den1, den1 * den2);
                t.shorten();
                t
            }
            (PrimNum::Int(v1), PrimNum::Float(v2)) | (PrimNum::Float(v2), PrimNum::Int(v1)) => {
                PrimNum::Float(v1 as f64 + v2)
            }
            (PrimNum::Rational(num1, den1), PrimNum::Int(v2))
            | (PrimNum::Int(v2), PrimNum::Rational(num1, den1)) => {
                let mut t = PrimNum::Rational(num1 + den1 * v2, den1);
                t.shorten();
                t
            }
            (PrimNum::Rational(num1, den1), PrimNum::Float(v2))
            | (PrimNum::Float(v2), PrimNum::Rational(num1, den1)) => {
                PrimNum::Float(num1 as f64 + den1 as f64 * v2 / den1 as f64)
            }
        }
    }
}

impl ops::Mul<PrimNum> for PrimNum {
    type Output = PrimNum;

    fn mul(self, rhs: PrimNum) -> PrimNum {
        match (self, rhs) {
            (PrimNum::Int(v1), PrimNum::Int(v2)) => PrimNum::Int(v1 * v2),
            (PrimNum::Float(v1), PrimNum::Float(v2)) => PrimNum::Float(v1 * v2),
            (PrimNum::Rational(num1, den1), PrimNum::Rational(num2, den2)) => {
                let mut t = PrimNum::Rational(num1 * num2, den1 * den2);
                t.shorten();
                t
            }
            (PrimNum::Int(v1), PrimNum::Float(v2)) | (PrimNum::Float(v2), PrimNum::Int(v1)) => {
                PrimNum::Float(v1 as f64 * v2)
            }
            (PrimNum::Rational(num1, den1), PrimNum::Int(v2))
            | (PrimNum::Int(v2), PrimNum::Rational(num1, den1)) => {
                let mut t = PrimNum::Rational(num1 * v2, den1);
                t.shorten();
                t
            }
            (PrimNum::Rational(num1, den1), PrimNum::Float(v2))
            | (PrimNum::Float(v2), PrimNum::Rational(num1, den1)) => {
                PrimNum::Float(num1 as f64 * v2 / den1 as f64)
            }
        }
    }
}

// Für Parser
impl ops::Mul<i128> for PrimNum {
    type Output = PrimNum;

    fn mul(self, rhs: i128) -> PrimNum {
        match self {
            PrimNum::Int(v) => PrimNum::Int(v * rhs),
            PrimNum::Float(v) => PrimNum::Float(v * rhs as f64),
            PrimNum::Rational(num, den) => PrimNum::Rational(num * rhs, den),
        }
    }
}

impl ops::Rem<i128> for PrimNum {
    type Output = PrimNum;

    fn rem(self, rhs: i128) -> PrimNum {
        match self {
            PrimNum::Int(v) => PrimNum::Int(v % rhs),
            PrimNum::Float(v) => PrimNum::Float(v % rhs as f64),
            PrimNum::Rational(num, den) => PrimNum::Float((num as f64 / den as f64) % rhs as f64),
        }
    }
}

impl PartialEq<i128> for PrimNum {
    fn eq(&self, other: &i128) -> bool {
        match self {
            PrimNum::Int(v) => v == other,
            PrimNum::Float(v) => *v == (*other as f64),
            _ => false,
        }
    }
}

impl PartialOrd<i128> for PrimNum {
    fn partial_cmp(&self, other: &i128) -> Option<Ordering> {
        Some(self.cmp(&PrimNum::Int(*other)))
    }

    fn lt(&self, other: &i128) -> bool {
        match self {
            PrimNum::Int(v) => v < other,
            PrimNum::Float(v) => *v < (*other as f64),
            PrimNum::Rational(num, den) => (*num as f64 / *den as f64) < (*other as f64),
        }
    }

    fn le(&self, other: &i128) -> bool {
        match self {
            PrimNum::Int(v) => v <= other,
            PrimNum::Float(v) => *v <= (*other as f64),
            PrimNum::Rational(num, den) => (*num as f64 / *den as f64) <= (*other as f64),
        }
    }

    fn gt(&self, other: &i128) -> bool {
        match self {
            PrimNum::Int(v) => v > other,
            PrimNum::Float(v) => *v > (*other as f64),
            PrimNum::Rational(num, den) => (*num as f64 / *den as f64) > (*other as f64),
        }
    }

    fn ge(&self, other: &i128) -> bool {
        match self {
            PrimNum::Int(v) => v >= other,
            PrimNum::Float(v) => *v >= (*other as f64),
            PrimNum::Rational(num, den) => (*num as f64 / *den as f64) >= (*other as f64),
        }
    }
}

impl Eq for PrimNum {}

impl Ord for PrimNum {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PrimNum::Int(_), PrimNum::Float(_))
            | (PrimNum::Int(_), PrimNum::Rational(_, _))
            | (PrimNum::Float(_), PrimNum::Rational(_, _)) => Ordering::Less,
            (PrimNum::Float(_), PrimNum::Int(_))
            | (PrimNum::Rational(_, _), PrimNum::Int(_))
            | (PrimNum::Rational(_, _), PrimNum::Float(_)) => Ordering::Greater,
            (PrimNum::Int(v1), PrimNum::Int(v2)) => v1.cmp(v2),
            (PrimNum::Float(v1), PrimNum::Float(v2)) => {
                if v1 > v2 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (PrimNum::Rational(num1, den1), PrimNum::Rational(num2, den2)) => {
                if *num1 as f64 / *den1 as f64 > *num2 as f64 / *den2 as f64 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

impl Hash for PrimNum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PrimNum::Int(v) => {
                v.hash(state);
            }
            PrimNum::Float(v) => {
                v.to_string().hash(state);
            }
            PrimNum::Rational(num, den) => {
                num.hash(state);
                den.hash(state);
            }
        }
    }
}
