use crate::types::{ast::Ast, NumberType};

/// Can expand the log function in a mathematical expression. log(x^a*y) = a*log(x)+log(y)
pub fn expand_log<N>(args: &Vec<Ast<N>>) -> Option<Ast<N>>
where
    N: NumberType,
{
    if args.len() == 1 {
        match &args[0] {
            Ast::Mul(vec) => {
                let mut result = vec![];
                for node in vec {
                    if let Ast::Pow(base, exp) = node {
                        result.push(Ast::Mul(vec![
                            *exp.clone(),
                            Ast::Func("log".to_owned(), vec![*base.clone()]),
                        ]));
                    } else {
                        result.push(Ast::Func("log".to_owned(), vec![node.clone()]));
                    }
                }

                Some(Ast::Add(result))
            }
            _ => None,
        }
    } else {
        None
    }
}
