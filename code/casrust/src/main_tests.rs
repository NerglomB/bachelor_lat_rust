use casrust::evaluator::base_evaluator;
use casrust::extensions::ast::SimplifyType;
use casrust::types::ast::Ast;
use casrust::types::prim_num::PrimNum;
use std::str::FromStr;

fn main() {
    let eval = base_evaluator();
    match Ast::from_str("1 + 2 + 3 + 4 - 5") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("1*2*3*4") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("1*2*3*4*0") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("1*2*3*4*(-1)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2+3*4+5") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(2+3)*(4+5)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2^(3^4)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("0.1+0.2") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("3^(-1) + 3^(-1)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("3^(-1) * 3^(-1)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(2)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(2)+sqrt(2)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(2)*sqrt(2)") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };

    match Ast::from_str("x+x+2*x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2*x-x-x+y") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x+x+a*x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("a*x-x-x+y") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x*0") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x*x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^2*x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^a*x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x/x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^2/x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^a/x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("a*x+b*x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("a^x*b^x") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^a*x^b") {
        Ok(ast) => {}
        Err(_) => {
            println!("error in term")
        }
    };

    match Ast::from_str("2+x") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(3)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2+x") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(-3)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2-x") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(-3)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2+x") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(0.5)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2^x + 2^x") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(2)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("2^x + 2^x") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(-1)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(x)") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(3)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(x)") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(4)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(16)") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(16)));
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("sqrt(17)") {
        Ok(ast) => {
            ast.simple_eval_sub(&eval, "x", &Ast::Num(PrimNum::from(17)));
        }
        Err(_) => {
            println!("error in term")
        }
    };

    match Ast::from_str("(x + 1)*(x - 2) - (x - 1)*x") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(a+b)*(c+d)*e") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(a+b)^3") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^(a+b)") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("x^(a*b)") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(x+y)^a") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(x*y)^a") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("log(x^2*y)") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("log(x^a*y)") {
        Ok(ast) => {
            ast.expand(&eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };

    match Ast::from_str("(x^2 + x)/x") {
        Ok(ast) => {
            ast.simplify(SimplifyType::Base, &eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(x^2 + x)*(y^2 + y)/x/y") {
        Ok(ast) => {
            ast.simplify(SimplifyType::Base, &eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(x^2 + x)*(y^2 + y)/x/a") {
        Ok(ast) => {
            ast.simplify(SimplifyType::Base, &eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
    match Ast::from_str("(x^3 + x^2 + x)*(x^2 + x)/x") {
        Ok(ast) => {
            ast.simplify(SimplifyType::Base, &eval);
        }
        Err(_) => {
            println!("error in term")
        }
    };
}
