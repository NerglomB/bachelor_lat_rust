//! # casrustlib
//!
//! `casrustlib` is a computer algebra system written in Rust. It aims to be similiar to SymPy, though many features are still missing.
//!
//! # Examples
//! ```
//! let a1 = Ast::Num(PrimNum::Float(0.1));
//! let a2 = Ast::Num(PrimNum::Float(0.2));
//! println!("{}", a1 + a2);
//!
//! let a1 = Ast::Num(PrecisionNum::Float(BigDecimal::from_str("0.1").unwrap()));
//! let a2 = Ast::Num(PrecisionNum::Float(BigDecimal::from_str("0.2").unwrap()));
//! println!("{}", a1 + a2);
//! ```
//! ```
//! let eval = base_evaluator();
//!
//! match Ast::from_str("a + b + c") {
//!   Ok(ast) => {
//!       for i_a in 0..50 {
//!           for i_b in 0..50 {
//!               for i_c in 0..50 {
//!                   ast.simple_eval_sub(&eval, "a", &Ast::Num(PrimNum::Int(i_a)))
//!                       .simple_eval_sub(&eval, "b", &Ast::Num(PrimNum::Int(i_b)))
//!                       .simple_eval_sub(&eval, "c", &Ast::Num(PrimNum::Int(i_c)));
//!               }
//!           }
//!       }
//!   }
//!   Err(_) => {
//!       println!("error in term");
//!   }
//! };
//! ```

pub mod evaluator;
pub mod extensions;
pub mod parser;
pub mod types;
