use crate::evaluator::{add_eval::*, mul_eval::*, EvalFn};
use crate::types::{ast::Ast, NumberType, Operator};
use std::collections::HashMap;

pub fn add<N>(vec: Vec<Ast<N>>, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N>
where
    N: NumberType,
{
    let mut result = Ast::Num(N::zero());
    let mut terms: HashMap<Ast<N>, Ast<N>> = HashMap::new();
    let mut extend_seq: Vec<Ast<N>> = vec![];
    for node in vec {
        extend_seq.append(&mut node.flatten(&Operator::Add));
    }
    for node in extend_seq {
        match &node {
            Ast::Num(_) | Ast::Add(_) => {
                result = result + node;
            }
            Ast::Mul(vec) => {
                add_mul(vec, &mut terms);
            }
            _ => {
                add_term(node, &mut terms);
            }
        }
    }

    for fun in evaler.adders.iter() {
        fun(&mut terms, hard_eval);
    }

    for (sym, val) in terms {
        match &val {
            Ast::Num(v) => {
                if *v == 1 {
                    result = result + sym;
                } else if *v != 0 {
                    result = result + (val * sym);
                }
            }
            _ => {
                result = result + (val * sym);
            }
        }
    }
    result.shorten().sort();

    result
}

pub fn mul<N>(vec: Vec<Ast<N>>, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N>
where
    N: NumberType,
{
    let mut result = Ast::Num(N::one());
    let mut terms: HashMap<Ast<N>, Ast<N>> = HashMap::new();
    let mut extend_seq: Vec<Ast<N>> = vec![];
    for node in vec {
        extend_seq.append(&mut node.flatten(&Operator::Mul));
    }
    for node in extend_seq {
        match &node {
            Ast::Num(_) | Ast::Mul(_) => {
                result = result * node;
            }
            Ast::Pow(base, exp) => {
                mul_pow(base, exp, &mut terms);
            }
            _ => {
                mul_term(node, &mut terms);
            }
        }
    }

    result.shorten();
    if result == Ast::Num(N::zero()) {
        return Ast::Num(N::zero());
    }

    for fun in evaler.muls.iter() {
        fun(&mut terms, hard_eval);
    }

    for (sym, val) in terms {
        match &val {
            Ast::Num(v) => {
                if *v == 0 {
                    return Ast::Num(N::one());
                } else if *v == 1 {
                    result = result * sym;
                } else {
                    result = result * Ast::Pow(Box::new(sym), Box::new(val));
                }
            }
            _ => {
                result = result * Ast::Pow(Box::new(sym), Box::new(val));
            }
        }
    }
    result.shorten().sort();

    result
}

pub fn func<N>(name: &str, args: Vec<Ast<N>>, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N>
where
    N: NumberType,
{
    let ret_val = if evaler.funcs.contains_key(name) {
        evaler.funcs[name](&args, hard_eval)
    } else {
        None
    };

    if ret_val.is_some() {
        ret_val.unwrap().eval(&evaler, hard_eval)
    } else {
        Ast::Func(String::from(name), args)
    }
}

pub fn pow<N>(base: Ast<N>, exp: Ast<N>, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N>
where
    N: NumberType,
{
    let (base, exp) = if let Ast::Pow(b, pow_base_box) = base {
        (*b, exp * *pow_base_box)
    } else {
        (base, exp)
    };
    let mut ret_val: Option<Ast<N>> = None;
    if let Ast::Num(exp_v) = exp.clone() {
        if exp_v.is_integer() {
            if exp_v == 0 {
                ret_val = Some(Ast::Num(N::one()));
            } else if exp_v == 1 {
                ret_val = Some(base.clone());
            } else if let Ast::Num(val) = base.clone() {
                if val.is_integer() {
                    if exp_v >= 0 {
                        ret_val = Some(Ast::Num(val.pow(exp_v)));
                    } else {
                        ret_val = Some(Ast::Num(N::try_create_rational(
                            N::one(),
                            val.pow(exp_v.abs()),
                        )));
                    }
                }
            }
        }
    }

    if ret_val == None {
        for fun in evaler.pows.iter() {
            if let Some(ast) = fun(&base, &exp, hard_eval) {
                ret_val = Some(ast.eval(evaler, hard_eval));
                break;
            }
        }
    }

    ret_val.unwrap_or(Ast::Pow(Box::new(base), Box::new(exp)))
}
