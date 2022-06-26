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

    #[test]
    fn test_sub() {
        match Ast::from_str("2+x") {
            Ok(ast) => {
                println!(
                    "2+x, x=3 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(3)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2+x") {
            Ok(ast) => {
                println!(
                    "2+x, x=-3 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(-3)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2-x") {
            Ok(ast) => {
                println!(
                    "2-x, x=-3 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(-3)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2+x") {
            Ok(ast) => {
                println!(
                    "2+x, x=0.5 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(0.5)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2^x + 2^x") {
            Ok(ast) => {
                println!(
                    "2^x + 2^x, x=2 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(2)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("2^x + 2^x") {
            Ok(ast) => {
                println!(
                    "2^x + 2^x, x=-1 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(-1)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(x)") {
            Ok(ast) => {
                println!(
                    "sqrt(x), x=3 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(3)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(x)") {
            Ok(ast) => {
                println!(
                    "sqrt(x), x=4 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(4)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(16)") {
            Ok(ast) => {
                println!(
                    "sqrt(x), x=16 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(16)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("sqrt(17)") {
            Ok(ast) => {
                println!(
                    "sqrt(x), x=17 = {}",
                    ast.simple_eval_sub(&base_evaluator(), "x", &Ast::Num(PrimNum::from(17)))
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
    }

    #[test]
    fn test_expand() {
        match Ast::from_str("(x + 1)*(x - 2) - (x - 1)*x") {
            Ok(ast) => {
                println!(
                    "(x + 1)*(x - 2) - (x - 1)*x = {}",
                    ast.expand(&base_evaluator())
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(a+b)*(c+d)*e") {
            Ok(ast) => {
                println!("(a+b)*(c+d)*e = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(a+b)^3") {
            Ok(ast) => {
                println!("(a+b)^3 = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^(a+b)") {
            Ok(ast) => {
                println!("x^(a+b) = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("x^(a*b)") {
            Ok(ast) => {
                println!("x^(a*b) = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(x+y)^a") {
            Ok(ast) => {
                println!("(x+y)^a = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(x*y)^a") {
            Ok(ast) => {
                println!("(x*y)^a = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("log(x^2*y)") {
            Ok(ast) => {
                println!("log(x^2*y) = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("log(x^a*y)") {
            Ok(ast) => {
                println!("log(x^a*y) = {}", ast.expand(&base_evaluator()));
            }
            Err(_) => {
                println!("error in term")
            }
        };
    }

    #[test]
    fn test_simplify() {
        match Ast::from_str("(x^2 + x)/x") {
            Ok(ast) => {
                println!(
                    "(x^2 + x)/x = {}",
                    ast.simplify(SimplifyType::Base, &base_evaluator())
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(x^2 + x)*(y^2 + y)/x/y") {
            Ok(ast) => {
                println!(
                    "(x^2 + x)*(y^2 + y)/x/y = {}",
                    ast.simplify(SimplifyType::Base, &base_evaluator())
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(x^2 + x)*(y^2 + y)/x/a") {
            Ok(ast) => {
                println!(
                    "(x^2 + x)*(y^2 + y)/x/a = {}",
                    ast.simplify(SimplifyType::Base, &base_evaluator())
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
        match Ast::from_str("(x^3 + x^2 + x)*(x^2 + x)/x") {
            Ok(ast) => {
                println!(
                    "(x^3 + x^2 + x)*(x^2 + x)/x = {}",
                    ast.simplify(SimplifyType::Base, &base_evaluator())
                );
            }
            Err(_) => {
                println!("error in term")
            }
        };
    }
}
