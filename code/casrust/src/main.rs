use casrust::evaluator::base_evaluator;
use casrust::extensions::ast::SimplifyType;
use casrust::parser::{lexer::Lexer, parser::Parser};
use casrust::types::ast::Ast;
use casrust::types::prim_num::PrimNum;
use std::str::FromStr;

fn main() {
    match Ast::from_str("(x^3+x^2+x+1)/x*(x^2+x)*(x^2+x)") {
        // match Ast::from_str("log(x^2*b)") {
        // match Ast::from_str("(x^3+x^2+x+1)/y") {
        Ok(ast) => {
            // println!("{}", ast.count_ops());
            let eval = base_evaluator();
            // println!("{}", ast);
            println!("{:?}", ast);
            println!("{}", ast.simplify(SimplifyType::Base, &eval));
            //println!("{}", ast.expand(&eval));
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

    enum BTree {
        Number(i64),
        Symbol(String),
        Add(Box<BTree>, Box<BTree>),
    }
    let tree = BTree::Add(
        Box::new(BTree::Add(
            Box::new(BTree::Symbol("x".to_owned())),
            Box::new(BTree::Number(1)),
        )),
        Box::new(BTree::Number(2)),
    );

    let mut leave_1 = BTree::Number(1);
    let mut leave_2 = BTree::Number(2);
    let mut parent_1 = BTree::Add(Box::new(leave_1), Box::new(leave_2));
    let mut refs = vec![&mut parent_1, &mut leave_1, &mut leave_2];
}
