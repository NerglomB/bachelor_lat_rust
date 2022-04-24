use crate::types::{ast::Ast, prim_num::PrimNum, NumberType};

pub fn func_sin<N>(name: &str, args: &Vec<Ast<N>>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType + SinCos,
{
    if name == "sin" {
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
    } else {
        None
    }
}

pub fn func_cos<N>(name: &str, args: &Vec<Ast<N>>, hard_eval: &bool) -> Option<Ast<N>>
where
    N: NumberType + SinCos,
{
    if name == "cos" {
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
    } else {
        None
    }
}

pub fn func_sqrt<N>(name: &str, args: &Vec<Ast<N>>, _: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val: Option<Ast<N>> = None;
    if name == "sqrt" && args.len() == 1 {
        ret_val = Some(Ast::Pow(
            Box::new(args[0].clone()),
            Box::new(Ast::Num(N::try_create_rational(N::one(), N::from(2)))),
        ));
    }

    ret_val
}

pub fn func_nthroot<N>(name: &str, args: &Vec<Ast<N>>, _: &bool) -> Option<Ast<N>>
where
    N: NumberType,
{
    let mut ret_val: Option<Ast<N>> = None;
    if name == "nthroot" && args.len() == 2 {
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
