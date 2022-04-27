use casrust::evaluator::base_evaluator;
use casrust::types::ast::Ast;
use casrust::types::prim_num::PrimNum;
use std::str::FromStr;

fn main() {
    match Ast::from_str("(x+1)*(x+2)") {
        Ok(ast) => {
            println!("{:?}", ast);
            let eval = base_evaluator();
            // println!(
            //     "{:?}",
            //     ast.simple_eval_sub(&eval, &Some("x"), &Some(Ast::Num(PrimNum::Int(0))))
            //         .simple_eval_sub(&eval, &Some("y"), &Some(Ast::Num(PrimNum::Int(3))))
            // );
            println!("{:?}", ast.expand());
            println!("{:?}", ast.expand().simple_eval(&eval));
        }
        Err(_) => {
            println!("error in term")
        }
    }
}
