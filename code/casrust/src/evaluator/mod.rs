pub mod add_eval;
pub mod common_eval;
pub mod function_eval;
pub mod mul_eval;
pub mod pow_eval;

use crate::evaluator::{add_eval::*, function_eval::*, pow_eval::*};
use crate::extensions::{function_expand::*, function_simplify::*};
use crate::types::{ast::Ast, constants::*, NumberType};
use std::collections::HashMap;

/// Wrapper for functions which are applied during evaluating a mathematical expression.
pub struct EvalFn<N> {
    pub adders: Vec<fn(&mut HashMap<Ast<N>, Ast<N>>, &bool)>,
    pub muls: Vec<fn(&mut HashMap<Ast<N>, Ast<N>>, &bool)>,
    pub pows: Vec<fn(&Ast<N>, &Ast<N>, &bool) -> Option<Ast<N>>>,
    pub funcs: HashMap<String, fn(&Vec<Ast<N>>, &bool) -> Option<Ast<N>>>,
    pub consts: HashMap<String, fn() -> Ast<N>>,
    pub expand_funcs: HashMap<String, fn(&Vec<Ast<N>>) -> Option<Ast<N>>>,
    pub simplify_funcs: Vec<fn(&Ast<N>) -> Ast<N>>,
}

/// Returns a basic evaluator. Can be extended.
pub fn base_evaluator<N>() -> EvalFn<N>
where
    N: NumberType + SinCos,
{
    let mut funcs: HashMap<String, fn(&Vec<Ast<N>>, &bool) -> Option<Ast<N>>> = HashMap::new();
    funcs.insert("sin".to_string(), func_sin);
    funcs.insert("cos".to_string(), func_cos);
    funcs.insert("sqrt".to_string(), func_sqrt);
    funcs.insert("nthroot".to_string(), func_nthroot);
    funcs.insert("limit".to_string(), func_limit);

    let mut consts: HashMap<String, fn() -> Ast<N>> = HashMap::new();
    consts.insert("π".to_string(), pi_const);
    consts.insert("∞".to_string(), infinity_const);

    let mut expand_funcs: HashMap<String, fn(&Vec<Ast<N>>) -> Option<Ast<N>>> = HashMap::new();
    expand_funcs.insert("log".to_string(), expand_log);

    EvalFn {
        adders: vec![add_sin_cos],
        muls: vec![],
        pows: vec![pow_mul, perfect_nth_root],
        funcs,
        consts,
        expand_funcs,
        simplify_funcs: vec![simplify_log],
    }
}
