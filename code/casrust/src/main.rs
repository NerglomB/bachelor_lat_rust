use bigdecimal::BigDecimal;
use casrust::types::ast::Ast;
use casrust::types::precision_num::PrecisionNum;
use casrust::types::prim_num::PrimNum;
use std::str::FromStr;

fn main() {
    let a1 = Ast::Num(PrimNum::Float(0.1));
    let a2 = Ast::Num(PrimNum::Float(0.2));
    println!("{}", a1 + a2);

    let a1 = Ast::Num(PrecisionNum::Float(BigDecimal::from_str("0.1").unwrap()));
    let a2 = Ast::Num(PrecisionNum::Float(BigDecimal::from_str("0.2").unwrap()));
    println!("{}", a1 + a2);
}
