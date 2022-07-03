use casrust::evaluator::base_evaluator;
use casrust::types::{ast::Ast, prim_num::PrimNum};
use std::str::FromStr;

fn main() {
    let eval = base_evaluator();

    match Ast::from_str("a * b * c") {
        Ok(ast) => {
            for i_a in 0..50 {
                for i_b in 0..50 {
                    for i_c in 0..50 {
                        ast.simple_eval_sub(&eval, "a", &Ast::Num(PrimNum::Int(i_a)))
                            .simple_eval_sub(&eval, "b", &Ast::Num(PrimNum::Int(i_b)))
                            .simple_eval_sub(&eval, "c", &Ast::Num(PrimNum::Int(i_c)));
                    }
                }
            }
        }
        Err(_) => {
            println!("error in term");
        }
    };
}
