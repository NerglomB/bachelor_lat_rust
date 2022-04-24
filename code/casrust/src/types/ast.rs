use crate::evaluator::{base_evaluator, base_funcs::*, EvalFn};
use crate::parser::{lexer::Lexer, parser::Parser};
use crate::types::{prim_num::PrimNum, NumberType};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Ast<N> {
    Add(Vec<Ast<N>>),
    Mul(Vec<Ast<N>>),
    Symbol(String),
    Const(String),
    Pow(Box<Ast<N>>, Box<Ast<N>>),
    Func(String, Vec<Ast<N>>),
    Num(N),
}

impl<N> Ast<N>
where
    N: NumberType,
{
    pub fn shorten(&mut self) -> &mut Self {
        match self {
            Ast::Add(v) => {
                v.retain(|element| {
                    if let Ast::Num(v) = element {
                        *v != 0
                    } else {
                        true
                    }
                });
                if v.len() == 0 {
                    *self = Ast::Num(N::zero())
                } else if v.len() == 1 {
                    *self = v.pop().unwrap()
                }
            }
            Ast::Mul(v) => {
                if v.len() == 1 {
                    // Falls nur neutrales Element vorhanden wird ansonsten 0 zurückgegeben
                    *self = v.pop().unwrap()
                } else {
                    v.retain(|element| {
                        if let Ast::Num(v) = element {
                            *v != 1
                        } else {
                            true
                        }
                    });
                    if v.len() == 0 {
                        *self = Ast::Num(N::zero())
                    } else if v.len() == 1 {
                        *self = v.pop().unwrap()
                    }
                }
            }
            Ast::Num(v) => {
                v.shorten();
            }
            _ => {}
        };

        self
    }

    pub fn sort(&mut self) -> &mut Self {
        match self {
            Ast::Add(vec) | Ast::Mul(vec) => {
                vec.sort();
            }
            _ => {}
        };

        self
    }

    pub fn simple_eval(&self, evaler: &EvalFn<N>) -> Ast<N> {
        self.eval(evaler, &false)
    }

    pub fn hard_eval(&self, evaler: &EvalFn<N>) -> Ast<N> {
        self.eval(evaler, &true)
    }

    pub fn eval(&self, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N> {
        match self {
            Ast::Add(vec) => add(
                vec.iter().map(|t| t.eval(evaler, hard_eval)).collect(),
                evaler,
                hard_eval,
            ),
            Ast::Mul(vec) => mul(
                vec.iter().map(|t| t.eval(evaler, hard_eval)).collect(),
                evaler,
                hard_eval,
            ),
            Ast::Pow(base, exp) => pow(
                base.eval(evaler, hard_eval),
                exp.eval(evaler, hard_eval),
                evaler,
                hard_eval,
            ),
            Ast::Func(name, args) => func(
                name,
                args.iter().map(|t| t.eval(evaler, hard_eval)).collect(),
                evaler,
                hard_eval,
            ),
            Ast::Const(name) if *hard_eval => {
                let mut ret = None;
                for const_struct in evaler.consts.iter() {
                    if let Some(v) = const_struct.eval(&name) {
                        ret = Some(v);
                        break;
                    }
                }

                ret.unwrap_or(self.clone())
            }
            _ => self.clone(),
        }
    }
}

impl<N> Eq for Ast<N> where N: PartialEq {}

impl<N> Hash for Ast<N>
where
    N: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Ast::Add(vec) => {
                vec.hash(state);
            }
            Ast::Mul(vec) => {
                vec.hash(state);
            }
            Ast::Pow(l, r) => {
                l.hash(state);
                r.hash(state);
            }
            Ast::Num(v) => {
                v.hash(state);
            }
            Ast::Symbol(v) | Ast::Const(v) => {
                v.hash(state);
            }
            Ast::Func(v, args) => {
                v.hash(state);
                args.hash(state);
            }
        }
    }
}

impl<N> Ord for Ast<N>
where
    N: Hash + PartialOrd + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Ast::Symbol(sym), Ast::Symbol(sym2)) = (self, other) {
            sym.cmp(sym2)
        } else if let (Ast::Const(c), Ast::Const(c2)) = (self, other) {
            c.cmp(c2)
        } else if let (Ast::Func(name, _), Ast::Func(name2, _)) = (self, other) {
            name.cmp(name2)
        } else {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let hash1 = hasher.finish();
            let mut hasher = DefaultHasher::new();
            other.hash(&mut hasher);
            let hash2 = hasher.finish();
            hash1.cmp(&hash2)
        }
    }
}

impl<N> ops::Add<Ast<N>> for Ast<N>
where
    N: NumberType,
{
    type Output = Ast<N>;

    fn add(self, rhs: Ast<N>) -> Ast<N> {
        match (self.clone(), rhs.clone()) {
            (Ast::Num(v1), Ast::Num(v2)) => Ast::Num(v1 + v2),
            (Ast::Add(_), Ast::Add(v2)) => {
                let mut ret = self;
                for node in v2 {
                    ret = ret + node;
                }

                ret
            }

            (Ast::Add(v1), Ast::Num(v2)) | (Ast::Num(v2), Ast::Add(v1)) => {
                let mut t_vec: Vec<Ast<N>> = vec![];
                let mut abs = Ast::Num(v2);
                for node in v1 {
                    match node {
                        Ast::Num(_) => abs = abs + node,
                        _ => t_vec.push(node),
                    }
                }
                t_vec.push(abs);
                Ast::Add(t_vec)
            }
            (Ast::Add(v), other) | (other, Ast::Add(v)) => {
                let mut v = v;
                v.push(other);
                Ast::Add(v)
            }
            _ => Ast::Add(vec![self, rhs]),
        }
    }
}

impl<N> ops::Mul<Ast<N>> for Ast<N>
where
    N: NumberType,
{
    type Output = Ast<N>;

    fn mul(self, rhs: Ast<N>) -> Ast<N> {
        match (self.clone(), rhs.clone()) {
            (Ast::Num(v1), Ast::Num(v2)) => Ast::Num(v1 * v2),
            (Ast::Mul(_), Ast::Mul(v2)) => {
                let mut ret = self;
                for node in v2 {
                    ret = ret * node;
                }

                ret
            }

            (Ast::Mul(v1), Ast::Num(v2)) | (Ast::Num(v2), Ast::Mul(v1)) => {
                let mut t_vec: Vec<Ast<N>> = vec![];
                let mut abs = Ast::Num(v2);
                for node in v1 {
                    match node {
                        Ast::Num(_) => abs = abs * node,
                        _ => t_vec.push(node),
                    }
                }
                t_vec.push(abs);
                Ast::Mul(t_vec)
            }
            (Ast::Mul(v), other) | (other, Ast::Mul(v)) => {
                let mut v = v;
                v.push(other);
                Ast::Mul(v)
            }
            _ => Ast::Mul(vec![self, rhs]),
        }
    }
}

#[derive(Debug)]
pub struct AstError;

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AstError while parsing.")
    }
}

impl Error for AstError {}

impl FromStr for Ast<PrimNum> {
    type Err = AstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let eval = base_evaluator();
        let tokens = Lexer::new(s).into_tokens();
        let ast = Parser { evaler: &eval }.parse(&tokens);

        Ok(ast.simple_eval(&eval))
    }
}
