use crate::evaluator::EvalFn;
use crate::types::ast::Ast;
use crate::types::NumberType;
use std::collections::{hash_map::Entry, HashMap};

impl<N> Ast<N>
where
    N: NumberType,
{
    pub fn expand(&self, evaler: &EvalFn<N>) -> Ast<N> {
        match self {
            Ast::Add(vec) => Ast::Add(vec.iter().map(|e| e.expand(evaler)).collect()),
            Ast::Mul(vec) => {
                let mut result: Vec<Ast<N>> = vec![];

                let v: Vec<Ast<N>> = vec.iter().map(|e| e.expand(evaler)).collect();
                for node in v {
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
                                for e in &mut result {
                                    *e = e.clone() * node.clone()
                                }
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

                if evaler.expand_funcs.contains_key(name) {
                    evaler.expand_funcs[name](&args).unwrap_or(Ast::Func(name.to_string(), args))
                } else {
                    Ast::Func(name.to_string(), args)
                }
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

                        Ast::Mul(mul).expand(evaler).simple_eval(evaler)
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

    pub fn simplify(&self, evaler: &EvalFn<N>) -> Ast<N> {
        self.simplify_ratio(1, evaler)
    }

    pub fn simplify_ratio(&self, ratio: u32, evaler: &EvalFn<N>) -> Ast<N> {
        let simplified = self.simplify_wrapper(evaler);

        if self.count_ops() == 0 || simplified.count_ops() / self.count_ops() >= ratio {
            self.clone()
        } else {
            simplified
        }
    }

    fn simplify_wrapper(&self, evaler: &EvalFn<N>) -> Ast<N> {
        match self {
            Ast::Mul(vec) => {
                let mut result: Vec<Ast<N>> = vec![];
                let mut t_result: Vec<Ast<N>> = vec![];

                let v: Vec<Ast<N>> = vec.iter().map(|e| e.simplify_wrapper(evaler)).collect();
                let mut pows: HashMap<Ast<N>, Ast<N>> = HashMap::new();
                for node in v {
                    match node {
                        Ast::Pow(base, exp) => match pows.entry(*exp) {
                            Entry::Occupied(mut o) => {
                                *o.get_mut() = o.get().clone() * *base;
                            }
                            Entry::Vacant(v) => {
                                v.insert(*base);
                            }
                        },
                        _ => t_result.push(node),
                    }
                }

                for (exp, base) in pows {
                    t_result.push(Ast::Pow(Box::new(base), Box::new(exp)));
                }

                // Summen und anderes aufsplitten
                let mut all_sums: Vec<Ast<N>> = vec![];
                let mut all_others: Vec<Ast<N>> = vec![];
                for node in t_result {
                    match node {
                        Ast::Add(_) => all_sums.push(node),
                        _ => all_others.push(node),
                    }
                }

                println!("{:?}", all_sums);
                println!("{:?}", all_others);

                // Alle summen for loop
                // innerhalb alle anderen for loop
                // für jedes Ersparnis im Gegensatz zum Original merken
                // das jeweils kleinste nehmen
                // wenn andere gefunden, dass kleiner, dann bisheriges merken und nochmal loopen

                if result.len() == 1 {
                    result.pop().unwrap()
                } else {
                    let mut result = Ast::Mul(result);
                    result.shorten().sort();

                    result
                }
            }
            _ => self.clone(),
        }
    }
}
