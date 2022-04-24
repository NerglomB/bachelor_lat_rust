use crate::types::{ast::Ast, NumberType};
use std::collections::{hash_map::Entry, HashMap};

pub fn mul_term<N>(node: Ast<N>, terms: &mut HashMap<Ast<N>, Ast<N>>)
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

pub fn mul_pow<N>(base: &Ast<N>, exp: &Ast<N>, terms: &mut HashMap<Ast<N>, Ast<N>>)
where
    N: NumberType,
{
    match terms.entry(base.clone()) {
        Entry::Occupied(mut o) => {
            *o.get_mut() = o.get().clone() + exp.clone();
        }
        Entry::Vacant(v) => {
            v.insert(exp.clone());
        }
    }
}
