pub mod add_eval;
pub mod common_eval;
pub mod function_eval;
pub mod mul_eval;
pub mod pow_eval;

use crate::evaluator::{add_eval::*, function_eval::*, pow_eval::*};
use crate::extensions::function_expand::*;
use crate::types::{
    ast::Ast,
    constants::{ConstType, PiConst},
    NumberType,
};
use std::collections::HashMap;

pub struct EvalFn<N> {
    pub adders: Vec<fn(&mut HashMap<Ast<N>, Ast<N>>, &bool)>,
    pub muls: Vec<fn(&mut HashMap<Ast<N>, Ast<N>>, &bool)>,
    pub pows: Vec<fn(&Ast<N>, &Ast<N>, &bool) -> Option<Ast<N>>>,
    pub funcs: Vec<fn(&str, &Vec<Ast<N>>, &bool) -> Option<Ast<N>>>,
    pub consts: Vec<Box<dyn ConstType<N>>>,
    pub expand_funcs: Vec<fn(&str, &Vec<Ast<N>>) -> Option<Ast<N>>>,
}

pub fn base_evaluator<N>() -> EvalFn<N>
where
    N: NumberType + SinCos,
{
    EvalFn {
        adders: vec![add_sin_cos],
        muls: vec![],
        pows: vec![pow_mul, perfect_nth_root],
        funcs: vec![func_sin, func_cos, func_sqrt, func_nthroot],
        consts: vec![Box::new(PiConst {})],
        expand_funcs: vec![expand_log],
    }
}
