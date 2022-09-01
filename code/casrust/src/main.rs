use bigdecimal::BigDecimal;
use casrust::evaluator::base_evaluator;
use casrust::extensions::ast::SimplifyType;
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

    match Ast::from_str("(4*x)^(1/2)") {
        Ok(ast) => {
            println!("{}", ast);
        }
        Err(_) => {
            println!("error in term");
        }
    };

    match Ast::from_str("limit(1/x, x, 0, pos)") {
        Ok(ast) => {
            println!("{}", ast);
        }
        Err(_) => {
            println!("error in term");
        }
    };

    match Ast::from_str("limit(1+2+x, x, 3, pos)") {
        Ok(ast) => {
            println!("{}", ast);
        }
        Err(_) => {
            println!("error in term");
        }
    };

    match Ast::from_str("2*(a + b)") {
        Ok(ast) => {
            println!("{}", ast.expand(&base_evaluator()));
        }
        Err(_) => {
            println!("error in term");
        }
    };

    match Ast::from_str("a^x * b^x") {
        Ok(ast) => {
            println!("{}", ast.simplify(SimplifyType::Exp, &base_evaluator()));
        }
        Err(_) => {
            println!("error in term");
        }
    };

    match Ast::from_str("a*log(x)+log(y)+2") {
        Ok(ast) => {
            println!("{}", ast.simplify(SimplifyType::Funcs, &base_evaluator()));
        }
        Err(_) => {
            println!("error in term");
        }
    };
}
