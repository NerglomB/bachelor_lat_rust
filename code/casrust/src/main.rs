use casrust::types::ast::Ast;
use std::str::FromStr;

fn main() {
    let ast = Ast::from_str("3/2 - 1").unwrap();
    println!("{:?}", ast);
}
