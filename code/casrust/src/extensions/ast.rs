use crate::evaluator::EvalFn;
use crate::types::ast::Ast;
use crate::types::NumberType;

impl<N> Ast<N>
where
    N: NumberType,
{
    pub fn expand(&self, evaler: &EvalFn<N>) -> Ast<N> {
        match self {
            Ast::Add(vec) => Ast::Add(vec.iter().map(|e| e.expand(evaler)).collect()),
            Ast::Mul(vec) => {
                let mut result: Vec<Ast<N>> = vec![];

                for node in vec {
                    match node {
                        Ast::Add(v_add) => {
                            if result.is_empty() {
                                result.append(&mut v_add.clone());
                            } else {
                                let mut t_res = vec![];
                                // Durch fnmut kann nicht derefenziert werden, deshalb clone
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
                    result.pop().unwrap()
                } else {
                    let mut result = Ast::Add(result);
                    result.shorten().sort();

                    result
                }
            }
            Ast::Func(name, args) => {
                let args = args.iter().map(|e| e.expand(evaler)).collect();
                let mut ret = None;
                for fun in evaler.expand_funcs.iter() {
                    if let Some(ast) = fun(name, &args) {
                        ret = Some(ast);
                        break;
                    }
                }

                ret.unwrap_or(Ast::Func(name.to_string(), args))
            }
            Ast::Pow(base, exp) => {
                let base = base.expand(evaler);
                let exp = exp.expand(evaler);

                match base {
                    Ast::Mul(vec) => {
                        let mut mul = vec![];
                        for node in vec {
                            mul.push(
                                Ast::Pow(Box::new(node), Box::new(exp.clone())).expand(evaler),
                            );
                        }

                        Ast::Mul(mul)
                    }
                    _ => match exp {
                        Ast::Num(exp) if exp.is_integer() && exp > 0 => {
                            let mut mul = vec![];
                            for _ in 0..exp.into() {
                                mul.push(base.clone());
                            }

                            Ast::Mul(mul).expand(evaler).simple_eval(evaler)
                        }
                        Ast::Add(exp) => {
                            let mut mul = vec![];
                            for node in exp {
                                mul.push(Ast::Pow(Box::new(base.clone()), Box::new(node)));
                            }

                            Ast::Mul(mul)
                        }
                        _ => Ast::Pow(Box::new(base), Box::new(exp)),
                    },
                }
            }
            _ => self.clone(),
        }
    }
}
