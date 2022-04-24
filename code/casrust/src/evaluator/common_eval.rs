use crate::evaluator::{add_eval::*, mul_eval::*, EvalFn};
use crate::types::{ast::Ast, NumberType};
use std::collections::HashMap;

pub fn add<N>(vec: Vec<Ast<N>>, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N>
where
    N: NumberType,
{
    let mut result = Ast::Num(N::zero());
    let mut terms: HashMap<Ast<N>, Ast<N>> = HashMap::new();
    for node in vec {
        match &node {
            Ast::Num(_) | Ast::Add(_) => {
                // Soll wirklich immer hinzugefügt werden? Evtl. anpassung, sodass Rational und Float coexisiteren?
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
    for node in vec {
        match &node {
            Ast::Num(_) | Ast::Mul(_) => {
                // Soll wirklich immer hinzugefügt werden? Evtl. anpassung, sodass Rational und Float coexisiteren?
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

    // Könnte von typ rational sein, also gemeinsamer nenner teilen
    result.shorten();

    for fun in evaler.adders.iter() {
        fun(&mut terms, hard_eval);
    }

    for (sym, val) in terms {
        match &val {
            Ast::Num(v) => {
                if *v == 0 {
                    result = Ast::Num(N::one());
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
    let mut ret_val = None;
    for fun in evaler.funcs.iter() {
        if let Some(ast) = fun(name, &args, hard_eval) {
            ret_val = Some(ast.eval(evaler, hard_eval));
            break;
        }
    }
    ret_val.unwrap_or(Ast::Func(String::from(name), args))
}

pub fn pow<N>(base: Ast<N>, exp: Ast<N>, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N>
where
    N: NumberType,
{
    // a^2^3 zusammenfasse zu a^6
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
                    // Nach num type abstrahieren?
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
