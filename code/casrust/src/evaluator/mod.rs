pub mod base_funcs;
pub mod eval_funcs_add;
pub mod eval_funcs_core;
pub mod eval_funcs_mul;
pub mod eval_funcs_pow;

use crate::evaluator::{eval_funcs_add::*, eval_funcs_core::*, eval_funcs_pow::*};
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
    }
}
