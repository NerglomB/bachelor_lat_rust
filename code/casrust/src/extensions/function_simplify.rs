use crate::types::{ast::Ast, NumberType};

/// Can simplify the log function in a mathematical expression. a*log(x)+log(y) = log(x^a*y)
pub fn simplify_log<N>(ast: &Ast<N>) -> Ast<N>
where
    N: NumberType,
{
    match ast {
        Ast::Mul(vec) => {
            let mut log_val: Option<Ast<N>> = None;
            let mut other: Vec<Ast<N>> = vec![];
            for node in vec {
                match node {
                    Ast::Func(name, val) if name == "log" && val.len() == 1 && log_val == None => {
                        log_val = Some(val[0].clone());
                    }
                    _ => {
                        other.push(node.clone());
                    }
                }
            }

            match log_val {
                Some(val) => {
                    let mut other = Ast::Mul(other);
                    other.shorten().sort();
                    Ast::Func(
                        "log".to_owned(),
                        vec![Ast::Pow(Box::new(val), Box::new(other))],
                    )
                }
                None => ast.clone(),
            }
        }
        Ast::Add(vec) => {
            let mut log_vals: Vec<Ast<N>> = vec![];
            let mut other: Vec<Ast<N>> = vec![];

            for node in vec {
                match node {
                    Ast::Func(name, val) if name == "log" && val.len() == 1 => {
                        log_vals.push(val[0].clone());
                    }
                    _ => {
                        other.push(node.clone());
                    }
                }
            }

            Ast::Add(other) + Ast::Func("log".to_owned(), vec![Ast::Mul(log_vals)])
        }
        _ => ast.clone(),
    }
}
