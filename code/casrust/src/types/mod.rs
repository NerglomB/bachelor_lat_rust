pub mod ast;
pub mod constants;
pub mod precision_num;
pub mod prim_num;

use crate::types::prim_num::PrimNum;
use std::hash::Hash;
use std::ops;

/// Tokens used for lexing a string.
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

/// Valid operators in a math expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// An abstract NumberType. Simplifies logic of calculations.
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
    /// Returns the neutral element of addition.
    fn zero() -> Self;
    /// Returns the neutral element of multiplication.
    fn one() -> Self;
    /// Tries to create a rational element. Certain conditions could be applied in implementations.
    fn try_create_rational(num: Self, den: Self) -> Self;
    /// Tries to make math expression short, for example removing neutral elements.
    fn shorten(&mut self);
    /// Calculates the power of self as base and exp.
    fn pow(&self, exp: Self) -> Self;
    /// Calculates the absolute value.
    fn abs(&self) -> Self;
    /// Checks if the type is an integer.
    fn is_integer(&self) -> bool;
    /// Checks if the type is a float.
    fn is_float(&self) -> bool;
    /// Checks if the type is an rational.
    fn is_rational(&self) -> bool;
    /// Get's the numerator of a rational, otherwise the value.
    fn get_numerator(&self) -> Self;
    /// Get's the denominator of a rational, otherwise 1.
    fn get_denominator(&self) -> Self;
}
