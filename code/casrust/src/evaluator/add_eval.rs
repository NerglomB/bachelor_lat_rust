use crate::types::{ast::Ast, NumberType};
use std::collections::{hash_map::Entry, HashMap};

/// Indicates that an expression is added one more time in a mathematical expression.
pub fn add_term<N>(node: Ast<N>, terms: &mut HashMap<Ast<N>, Ast<N>>)
where
    N: NumberType,
{
    match terms.entry(node) {
        Entry::Occupied(mut o) => {
            *o.get_mut() = o.get().clone() + Ast::Num(N::one());
        }
        Entry::Vacant(v) => {
            v.insert(Ast::Num(N::one()));
        }
    }
}

/// Indicates that a multiplication expression is added n more times in a addition expression where n is a number found in the multiplication or 1 if not.
pub fn add_mul<N>(mul_vec: &Vec<Ast<N>>, terms: &mut HashMap<Ast<N>, Ast<N>>)
where
    N: NumberType,
{
    let mut t_vec: Vec<Ast<N>> = vec![];
    let mut coeff = Ast::Num(N::zero());
    for b in mul_vec.into_iter() {
        match b {
            Ast::Num(coeff_add) => {
                coeff = coeff + Ast::Num(coeff_add.clone());
            }
            _ => {
                t_vec.push(b.clone());
            }
        }
    }
    if let Ast::Num(v) = &coeff {
        if *v == 0 {
            coeff = Ast::Num(N::one());
        }
    }

    t_vec.sort();
    if t_vec.len() == 1 {
        match terms.entry(t_vec[0].clone()) {
            Entry::Occupied(mut o) => {
                *o.get_mut() = o.get().clone() + coeff;
            }
            Entry::Vacant(v) => {
                v.insert(coeff);
            }
        }
    } else {
        match terms.entry(Ast::Mul(t_vec)) {
            Entry::Occupied(mut o) => {
                *o.get_mut() = o.get().clone() + coeff;
            }
            Entry::Vacant(v) => {
                v.insert(coeff);
            }
        }
    }
}

/// Tries to add sin and cos function when possible and shorten it to 1.
pub fn add_sin_cos<N>(terms: &mut HashMap<Ast<N>, Ast<N>>, _hard_eval: &bool)
where
    N: NumberType,
{
    let mut cos_key: Option<Ast<N>> = None;
    let mut sin_key: Option<Ast<N>> = None;
    let mut cos_value: Option<Ast<N>> = None;
    let mut sin_value: Option<Ast<N>> = None;
    let mut cos_times: Option<Ast<N>> = None;
    let mut sin_times: Option<Ast<N>> = None;
    for (sym, val) in terms.clone() {
        match sym.clone() {
            Ast::Pow(base, exp) => match *exp {
                Ast::Num(v) if v == 2 => {
                    if let Ast::Func(name, value) = *base {
                        if value.len() == 1 {
                            if name == "cos" {
                                cos_key = Some(sym);
                                cos_value = Some(value[0].clone());
                                cos_times = Some(val);
                            } else if name == "sin" {
                                sin_key = Some(sym);
                                sin_value = Some(value[0].clone());
                                sin_times = Some(val);
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    if cos_key != None && sin_key != None && cos_value == sin_value && cos_times == sin_times {
        let times = cos_times.unwrap();
        if terms.contains_key(&times) {
            terms.insert(
                times.clone(),
                terms.get(&times).unwrap().clone() + Ast::Num(N::one()),
            );
        } else {
            terms.insert(times, Ast::Num(N::one()));
        }
        terms.remove(&cos_key.unwrap());
        terms.remove(&sin_key.unwrap());
    }
}
