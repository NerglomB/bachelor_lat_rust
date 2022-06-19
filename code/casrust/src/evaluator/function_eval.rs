use crate::evaluator::base_evaluator;
use crate::types::{ast::Ast, prim_num::PrimNum, NumberType};

pub fn func_sin<N>(args: &Vec<Ast<N>>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType + SinCos,
{
    if args.len() == 1 {
        match &args[0] {
            Ast::Num(v) if v.is_integer() || v.is_float() => {
                if *hard_eval {
                    Some(Ast::Num(v.sin()))
                } else {
                    if *v == 0 {
                        Some(Ast::Num(N::zero()))
                    } else if v.clone() % 90 == 0 {
                        Some(Ast::Num(N::one()))
                    } else {
                        None
                    }
                }
            }
            Ast::Num(v) if v.is_rational() => {
                if *hard_eval {
                    Some(Ast::Num(v.sin()))
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

pub fn func_cos<N>(args: &Vec<Ast<N>>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType + SinCos,
{
    if args.len() == 1 {
        match &args[0] {
            Ast::Num(v) if v.is_integer() || v.is_float() => {
                if *hard_eval {
                    Some(Ast::Num(v.cos()))
                } else {
                    if *v == 0 {
                        Some(Ast::Num(N::one()))
                    } else if v.clone() % 90 == 0 {
                        Some(Ast::Num(N::zero()))
                    } else {
                        None
                    }
                }
            }
            Ast::Num(v) if v.is_rational() => {
                if *hard_eval {
                    Some(Ast::Num(v.cos()))
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

pub fn func_sqrt<N>(args: &Vec<Ast<N>>, _: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val: Option<Ast<N>> = None;
    if args.len() == 1 {
        ret_val = Some(Ast::Pow(
            Box::new(args[0].clone()),
            Box::new(Ast::Num(N::try_create_rational(N::one(), N::from(2)))),
        ));
    }

    ret_val
}

pub fn func_nthroot<N>(args: &Vec<Ast<N>>, _: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val: Option<Ast<N>> = None;
    if args.len() == 2 {
        if let Ast::Num(nthroot) = &args[1] {
            if nthroot >= &0 {
                ret_val = Some(Ast::Pow(
                    Box::new(args[0].clone()),
                    Box::new(Ast::Num(N::try_create_rational(N::one(), nthroot.clone()))),
                ));
            }
        }
    }

    ret_val
}

pub trait SinCos {
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
}

impl SinCos for PrimNum {
    fn sin(&self) -> Self {
        match self {
            PrimNum::Int(v) => PrimNum::Float((*v as f64).sin()),
            PrimNum::Float(v) => PrimNum::Float(v.sin()),
            PrimNum::Rational(num, den) => PrimNum::Float((*num as f64 / *den as f64).sin()),
        }
    }

    fn cos(&self) -> Self {
        match self {
            PrimNum::Int(v) => PrimNum::Float((*v as f64).cos()),
            PrimNum::Float(v) => PrimNum::Float(v.cos()),
            PrimNum::Rational(num, den) => PrimNum::Float((*num as f64 / *den as f64).cos()),
        }
    }
}

pub fn func_limit<N>(args: &Vec<Ast<N>>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType + SinCos,
{
    let mut ret_val: Option<Ast<N>> = None;
    if args.len() == 4 {
        match &args[0] {
            Ast::Pow(base, exp) if **base == args[1] && **exp == Ast::Num(N::from(-1)) => {
                if args[2] == Ast::Num(N::from(1)) {
                    ret_val = Some(Ast::Num(N::from(1)));
                } else if args[2] == Ast::Const("∞".to_owned())
                    || args[2] == Ast::Mul(vec![Ast::Const("∞".to_owned()), Ast::Num(N::from(-1))])
                {
                    ret_val = Some(Ast::Num(N::from(0)));
                } else if args[2] == Ast::Num(N::from(0)) {
                    if args[3] == Ast::Symbol("pos".to_owned()) {
                        ret_val = Some(Ast::Const("∞".to_owned()));
                    } else if args[3] == Ast::Symbol("neg".to_owned()) {
                        ret_val = Some(Ast::Mul(vec![
                            Ast::Num(N::from(-1)),
                            Ast::Const("∞".to_owned()),
                        ]));
                    }
                } else if let Ast::Num(n) = &args[2] {
                    ret_val = Some(Ast::Num(N::try_create_rational(N::from(1), n.clone())));
                }
            }
            Ast::Pow(base, exp) if **base == args[1] && **exp == args[1] => {
                if let Ast::Num(n) = &args[2] {
                    if *n == 0 && args[3] == Ast::Symbol("pos".to_owned()) {
                        ret_val = Some(Ast::Num(N::one()));
                    }
                }
            }
            Ast::Symbol(sym) if Ast::Symbol(sym.clone()) == args[1] => {
                ret_val = Some(args[2].clone());
            }
            Ast::Mul(vec) => {
                let mut result = Ast::Mul(
                    vec.iter()
                        .map(|t| {
                            if let Some(l) = func_limit(
                                &vec![t.clone(), args[1].clone(), args[2].clone(), args[3].clone()],
                                hard_eval,
                            ) {
                                l
                            } else {
                                t.clone()
                            }
                        })
                        .collect(),
                )
                .eval(&base_evaluator(), hard_eval);
                result.shorten();
                ret_val = Some(result);
            }
            Ast::Add(vec) => {
                let mut result = Ast::Add(
                    vec.iter()
                        .map(|t| {
                            if let Some(l) = func_limit(
                                &vec![t.clone(), args[1].clone(), args[2].clone(), args[3].clone()],
                                hard_eval,
                            ) {
                                l
                            } else {
                                t.clone()
                            }
                        })
                        .collect(),
                )
                .eval(&base_evaluator(), hard_eval);
                result.shorten();
                ret_val = Some(result);
            }
            _ => {}
        }
    }

    ret_val
}
