use casrust::types::ast::Ast;
use std::str::FromStr;

fn main() {
    let mut expr_list = vec![];
    for i_a in 0..50 {
        for i_b in 0..50 {
            for i_c in 0..50 {
                match Ast::from_str("a + b + c") {
                    Ok(ast) => {
                        expr_list.push(ast);
                    }
                    Err(_) => {
                        println!("error in term")
                    }
                };
            }
        }
    }
}
