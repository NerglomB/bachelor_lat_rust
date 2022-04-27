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
            Ast::Pow(base, exp) => {
                let base = base.expand();
                let exp = exp.expand();

                match exp {
                    Ast::Num(exp) if exp.is_integer() && exp > 0 => {
                        let mut mul = vec![];
                        for _ in 0..exp.into() {
                            mul.push(base.clone());
                        }

                        Ast::Mul(mul).expand()
                    }
                    Ast::Add(exp) => {
                        let mut mul = vec![];
                        for node in exp {
                            mul.push(Ast::Pow(Box::new(base.clone()), Box::new(node)));
                        }

                        Ast::Mul(mul).expand()
                    }
                    _ => Ast::Pow(Box::new(base), Box::new(exp)),
                }
            }
            _ => self.clone(),
        }
    }
}
