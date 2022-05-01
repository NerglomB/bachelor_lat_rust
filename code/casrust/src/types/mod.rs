pub mod ast;
pub mod constants;
pub mod precision_num;
pub mod prim_num;

use crate::types::prim_num::PrimNum;
use std::hash::Hash;
use std::ops;

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Comma,
    Var(String),
    Func(String),
    Op(Operator),
    Num(PrimNum),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub trait NumberType:
    std::fmt::Debug
    + Clone
    + Hash
    + Eq
    + Ord
    + PartialEq<i128>
    + PartialOrd<i128>
    + ops::Add<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Rem<i128, Output = Self>
    + Into<i128>
    + From<i128>
    + From<f64>
    + std::fmt::Display
{
    fn zero() -> Self;
    fn one() -> Self;
    fn try_create_rational(num: Self, den: Self) -> Self;
    fn shorten(&mut self);
    fn pow(&self, exp: Self) -> Self;
    fn abs(&self) -> Self;
    fn is_integer(&self) -> bool;
    fn is_float(&self) -> bool;
    fn is_rational(&self) -> bool;
    fn get_numerator(&self) -> Self;
    fn get_denominator(&self) -> Self;
}
