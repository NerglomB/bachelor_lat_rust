use crate::types::ast::Ast;
use crate::types::NumberType;

impl<N> Ast<N>
where
    N: NumberType,
{
    pub fn expand(&self) -> Ast<N> {
        match self {
            Ast::Mul(vec) => {
                let mut result: Vec<Ast<N>> = vec![];

                for node in vec {
                    match node {
                        Ast::Add(v_add) => {
                            if result.is_empty() {
                                result.append(&mut v_add.clone());
                            } else {
                                let mut t_res = vec![];
                                for el in v_add {
                                    t_res.append(
                                        &mut result
                                            .clone()
                                            .into_iter()
                                            .map(|e| e * el.clone())
                                            .collect(),
                                    );
                                }
                                result = t_res;
                            }
                        }
                        _ => {
                            if result.is_empty() {
                                result.push(node.clone());
                            } else {
                                result = result.into_iter().map(|e| e * node.clone()).collect();
                            }
                        }
                    };
                }

                if result.len() == 1 {
                    result.pop().unwrap().sort().clone()
                } else {
                    Ast::Add(
                        result
                            .into_iter()
                            .map(|mut e| e.shorten().sort().clone())
                            .collect(),
                    )
                    .sort()
                    .clone()
                }
            }
            Ast::Add(vec) => Ast::Add(vec.iter().map(|e| e.expand()).collect()),
            Ast::Func(name, args) => {
                Ast::Func(name.to_string(), args.iter().map(|e| e.expand()).collect())
            }
            _ => self.clone(),
        }
    }
}
