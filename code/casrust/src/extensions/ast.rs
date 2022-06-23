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
            Ast::Add(vec) => {
                Ast::Add(vec.iter().map(|e| e.expand(evaler)).collect()).simple_eval(evaler)
            }
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

                    result.simple_eval(evaler)
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

    pub fn simplify(&self, s_type: SimplifyType, evaler: &EvalFn<N>) -> Ast<N> {
        self.simplify_ratio(s_type, 1.0, evaler)
    }

    pub fn simplify_ratio(&self, s_type: SimplifyType, ratio: f64, evaler: &EvalFn<N>) -> Ast<N> {
        let simplified = self.simplify_wrapper(s_type, evaler);

        if self.count_ops() == 0 || simplified.count_ops() as f64 / self.count_ops() as f64 > ratio
        {
            self.clone()
        } else {
            simplified
        }
    }

    fn simplify_wrapper(&self, s_type: SimplifyType, evaler: &EvalFn<N>) -> Ast<N> {
        match self {
            Ast::Mul(vec) => {
                let mut result: Vec<Ast<N>> = vec![];
                let v: Vec<Ast<N>> = vec
                    .iter()
                    .map(|e| e.simplify_wrapper(s_type, evaler))
                    .collect();

                match s_type {
                    SimplifyType::Exp => {
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
                                _ => result.push(node),
                            }
                        }

                        for (exp, base) in pows {
                            result.push(Ast::Pow(Box::new(base), Box::new(exp)));
                        }
                    }
                    SimplifyType::Base => {
                        let mut all_sums: Vec<Ast<N>> = vec![];
                        let mut all_others: Vec<Ast<N>> = vec![];
                        for node in v {
                            match node {
                                Ast::Add(_) => all_sums.push(node),
                                _ => all_others.push(node),
                            }
                        }
                        let mut combinations = vec![all_others.len(); all_sums.len()];
                        let mut combinations_save_count = vec![0; all_sums.len()];
                        let mut sums_visited = vec![false; all_sums.len()];

                        while sums_visited.contains(&false) {
                            let idx = sums_visited.iter().position(|&x| x == false).unwrap();
                            let base_count = all_sums[idx].count_ops() + 2;
                            for (other_idx, other) in all_others.iter().enumerate() {
                                let save_count = (all_sums[idx].clone() * other.clone())
                                    .expand(evaler)
                                    .simple_eval(evaler)
                                    .count_ops();
                                if save_count < base_count {
                                    let save_count = base_count - save_count;

                                    if save_count > combinations_save_count[idx]
                                        || combinations_save_count[idx] == 0
                                    {
                                        match combinations.iter().position(|&x| x == other_idx) {
                                            Some(before_idx)
                                                if save_count
                                                    > combinations_save_count[before_idx] =>
                                            {
                                                combinations_save_count[idx] = save_count;
                                                combinations[idx] = other_idx;
                                                combinations_save_count[before_idx] = 0;
                                                combinations[before_idx] = all_others.len();
                                            }
                                            None => {
                                                combinations_save_count[idx] = save_count;
                                                combinations[idx] = other_idx;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }

                            sums_visited[idx] = true;
                        }

                        for (idx, val) in combinations.iter().enumerate() {
                            if val < &all_others.len() {
                                result.push(
                                    (all_sums[idx].clone() * all_others[*val].clone())
                                        .expand(evaler)
                                        .simple_eval(evaler),
                                );
                            } else {
                                result.push(all_sums[idx].clone());
                            }
                        }

                        for (idx, _) in all_others.iter().enumerate() {
                            if let None = combinations.iter().position(|&x| x == idx) {
                                result.push(all_others[idx].clone());
                            }
                        }
                    }
                    SimplifyType::Funcs => {
                        let mut simplifier: Ast<N> = self.clone();
                        for fun in evaler.simplify_funcs.iter() {
                            simplifier = fun(&simplifier);
                        }

                        result.push(simplifier);
                    }
                }

                if result.len() == 1 {
                    result.pop().unwrap()
                } else {
                    let mut result = Ast::Mul(result);
                    result.shorten().sort();

                    result
                }
            }
            Ast::Add(vec) => {
                let res = Ast::Add(
                    vec.iter()
                        .map(|e| e.simplify_wrapper(s_type, evaler))
                        .collect(),
                );

                match s_type {
                    SimplifyType::Funcs => {
                        let mut simplifier: Ast<N> = res;
                        for fun in evaler.simplify_funcs.iter() {
                            simplifier = fun(&simplifier);
                        }

                        simplifier
                    }
                    _ => res,
                }
            }
            _ => self.clone(),
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
pub enum SimplifyType {
    Base,
    Exp,
    Funcs,
}
