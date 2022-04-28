use crate::types::{ast::Ast, NumberType};

pub fn expand_log<N>(name: &str, args: &Vec<Ast<N>>) -> Option<Ast<N>>
where
    N: NumberType,
{
    if name == "log" && args.len() == 1 {
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
