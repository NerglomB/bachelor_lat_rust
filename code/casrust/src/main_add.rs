use casrust::types::ast::Ast;
use std::str::FromStr;

fn main() {
    for i_a in 0..50 {
        for i_b in 0..50 {
            for i_c in 0..50 {
                match Ast::from_str("a + b + c") {
                    Ok(ast) => {}
                    Err(_) => {
                        println!("error in term")
                    }
                };
            }
        }
    }
}
