use casrust::evaluator::base_evaluator;
use casrust::parser::{lexer::Lexer, parser::Parser};
use casrust::types::ast::Ast;
use casrust::types::prim_num::PrimNum;
use std::str::FromStr;

fn main() {
    match Ast::from_str("(x*y)^z") {
        Ok(ast) => {
            let eval = base_evaluator();
            println!("{:?}", ast);
            println!("{:?}", ast.expand(&eval));
            // println!(
            //     "{:?}",
            //     ast.simple_eval_sub(&eval, &Some("x"), &Some(Ast::Num(PrimNum::Int(0))))
            //         .simple_eval_sub(&eval, &Some("y"), &Some(Ast::Num(PrimNum::Int(3))))
            // );
            // println!("{:?}", ast.expand(&eval));
            // println!(
            //     "{:?}",
            //     (ast.simple_eval(&eval) * ast.simple_eval(&eval)).simple_eval(&eval)
            // );
            // println!("{:?}", ast.hard_eval(&eval) * ast.hard_eval(&eval));
        }
        Err(_) => {
            println!("error in term")
        }
    };

    // match Ast::from_str_hard("sqrt(2)*sqrt(2)") {
    //     Ok(ast) => {
    //         println!("{:?}", ast);
    //     }
    //     Err(_) => {
    //         println!("error in term")
    //     }
    // };
}
