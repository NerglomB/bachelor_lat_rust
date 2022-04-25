use casrust::evaluator::base_evaluator;
use casrust::types::ast::Ast;
use casrust::types::prim_num::PrimNum;
use std::str::FromStr;

fn main() {
    let ast = Ast::from_str("x+(y*x)").unwrap();
    println!("{:?}", ast);
    let eval = base_evaluator();
    println!(
        "{:?}",
        ast.simple_eval_sub(&eval, &Some("x"), &Some(Ast::Num(PrimNum::Int(0))))
            .simple_eval_sub(&eval, &Some("y"), &Some(Ast::Num(PrimNum::Int(3))))
    );
}
