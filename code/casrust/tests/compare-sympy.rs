// cargo test -- --nocapture --test-threads=1

#[cfg(test)]
mod comparesympy {
    use casrust::evaluator::base_evaluator;
    use casrust::extensions::ast::SimplifyType;
    use casrust::parser::{lexer::Lexer, parser::Parser};
    use casrust::types::ast::Ast;
    use casrust::types::prim_num::PrimNum;
    use std::str::FromStr;

    #[test]
    fn test_calc() {
        match Ast::from_str("1 + 2 + 3 + 4 - 5") {
            Ok(ast) => {
                println!("1 + 2 + 3 + 4 - 5 = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("1*2*3*4") {
            Ok(ast) => {
                println!("1*2*3*4 = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("1*2*3*4*0") {
            Ok(ast) => {
                println!("1*2*3*4*0 = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("1*2*3*4*(-1)") {
            Ok(ast) => {
                println!("1*2*3*4*(-1) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2+3*4+5") {
            Ok(ast) => {
                println!("2+3*4+5 = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(2+3)*(4+5)") {
            Ok(ast) => {
                println!("(2+3)*(4+5) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2^(3^4)") {
            Ok(ast) => {
                println!("2^(3^4) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("0.1+0.2") {
            Ok(ast) => {
                println!("0.1+0.2 = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("3^(-1) + 3^(-1)") {
            Ok(ast) => {
                println!("3^(-1) + 3^(-1) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("3^(-1) * 3^(-1)") {
            Ok(ast) => {
                println!("3^(-1) * 3^(-1) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(2)") {
            Ok(ast) => {
                println!("sqrt(2) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(2)+sqrt(2)") {
            Ok(ast) => {
                println!("sqrt(2)+sqrt(2) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(2)*sqrt(2)") {
            Ok(ast) => {
                println!("sqrt(2)*sqrt(2) = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
    }

    #[test]
    fn test_variables() {
        match Ast::from_str("x+x+2*x") {
            Ok(ast) => {
                println!("x+x+2*x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2*x-x-x+y") {
            Ok(ast) => {
                println!("2*x-x-x+y = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x+x+a*x") {
            Ok(ast) => {
                println!("x+x+a*x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("a*x-x-x+y") {
            Ok(ast) => {
                println!("a*x-x-x+y = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x*0") {
            Ok(ast) => {
                println!("x*0 = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x*x") {
            Ok(ast) => {
                println!("x*x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^2*x") {
            Ok(ast) => {
                println!("x^2*x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^a*x") {
            Ok(ast) => {
                println!("x^a*x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x/x") {
            Ok(ast) => {
                println!("x/x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^2/x") {
            Ok(ast) => {
                println!("x^2/x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^a/x") {
            Ok(ast) => {
                println!("x^a/x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("a*x+b*x") {
            Ok(ast) => {
                println!("a*x+b*x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("a^x*b^x") {
            Ok(ast) => {
                println!("a^x*b^x = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^a*x^b") {
            Ok(ast) => {
                println!("x^a*x^b = {}", ast);
            }
            Err(_) => {
                println!("error in term")
            }
        };
    }
}
