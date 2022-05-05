use crate::evaluator::{base_evaluator, common_eval::*, EvalFn};
use crate::parser::{lexer::Lexer, parser::Parser};
use crate::types::{prim_num::PrimNum, NumberType, Operator};
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
    Pow(Box<Ast<N>>, Box<Ast<N>>),
    Symbol(String),
    Const(String),
    Func(String, Vec<Ast<N>>),
    Num(N),
}

impl<N> Ast<N>
where
    N: NumberType,
{
    pub fn flatten(&self, op: &Operator) -> Vec<Ast<N>> {
        match self {
            Ast::Add(vec) => {
                if Operator::Add == *op {
                    vec.clone().into_iter().collect()
                } else {
                    vec![self.clone()]
                }
            }
            Ast::Mul(vec) => {
                if Operator::Mul == *op {
                    vec.clone().into_iter().collect()
                } else {
                    vec![self.clone()]
                }
            }
            _ => vec![self.clone()],
        }
    }

    // Hier das problem mit rückgabetyp, versuche diesen in extension/ast bei rückgabe in mul anzuwenden -> fehler, clone nötig
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

    pub fn simple_eval_sub(&self, evaler: &EvalFn<N>, sub: &str, with: &Ast<N>) -> Ast<N> {
        self.eval_sub(evaler, &false, &Some(sub), &Some(with))
    }

    pub fn hard_eval(&self, evaler: &EvalFn<N>) -> Ast<N> {
        self.eval(evaler, &true)
    }

    pub fn hard_eval_sub(&self, evaler: &EvalFn<N>, sub: &str, with: &Ast<N>) -> Ast<N> {
        self.eval_sub(evaler, &true, &Some(sub), &Some(with))
    }

    pub fn eval(&self, evaler: &EvalFn<N>, hard_eval: &bool) -> Ast<N> {
        self.eval_sub(evaler, hard_eval, &None, &None)
    }

    pub fn eval_sub(
        &self,
        evaler: &EvalFn<N>,
        hard_eval: &bool,
        sub: &Option<&str>,
        with: &Option<&Ast<N>>,
    ) -> Ast<N> {
        match self {
            Ast::Add(vec) => add(
                vec.iter()
                    .map(|t| t.eval_sub(evaler, hard_eval, sub, with))
                    .collect(),
                evaler,
                hard_eval,
            ),
            Ast::Mul(vec) => mul(
                vec.iter()
                    .map(|t| t.eval_sub(evaler, hard_eval, sub, with))
                    .collect(),
                evaler,
                hard_eval,
            ),
            Ast::Pow(base, exp) => pow(
                base.eval_sub(evaler, hard_eval, sub, with),
                exp.eval_sub(evaler, hard_eval, sub, with),
                evaler,
                hard_eval,
            ),
            Ast::Func(name, args) => func(
                name,
                args.iter()
                    .map(|t| t.eval_sub(evaler, hard_eval, sub, with))
                    .collect(),
                evaler,
                hard_eval,
            ),
            Ast::Const(name) if *hard_eval => {
                if evaler.consts.contains_key(name) {
                    evaler.consts[name].eval(&name)
                } else {
                    self.clone()
                }
            }
            Ast::Symbol(name) if sub.is_some() && name == sub.unwrap() => with.unwrap().clone(),
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

        Ok(Parser { evaler: &eval }.parse(&tokens)?.simple_eval(&eval))
    }
}

impl Ast<PrimNum> {
    pub fn from_str_hard(s: &str) -> Result<Self, AstError> {
        let eval = base_evaluator();
        let tokens = Lexer::new(s).into_tokens();

        Ok(Parser { evaler: &eval }.parse(&tokens)?.hard_eval(&eval))
    }

    pub fn from_str_none(s: &str) -> Result<Self, AstError> {
        let eval = base_evaluator();
        let tokens = Lexer::new(s).into_tokens();

        Ok(Parser { evaler: &eval }.parse(&tokens)?)
    }
}

impl<N> std::fmt::Display for Ast<N>
where
    N: NumberType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            Ast::Add(vec) => {
                let mut vec = vec.clone();
                vec.sort_by(display_sort);
                let mut s = "".to_owned();
                let len = vec.len();
                let mut current = 0;
                while current < len {
                    if current == 0 {
                        s.push_str(&format!("{}", vec[current]));
                    } else {
                        let el_s = format!("{}", vec[current]);
                        if el_s.chars().nth(0).unwrap() == '-' {
                            s = s[0..s.len() - 1].to_string();
                        }
                        s.push_str(&el_s);
                    }
                    if current < len - 1 {
                        s.push_str("+");
                    }
                    current += 1;
                }

                write!(f, "{}", s)
            }
            Ast::Mul(vec) => {
                let mut s = "".to_owned();
                let len = vec.len();
                let mut current = 0;

                let mut vec = if let Ast::Num(_) = vec[len - 1] {
                    let mut new_v = vec![];
                    new_v.push(vec[len - 1].clone());
                    new_v.extend_from_slice(&vec[0..len - 1]);

                    new_v
                } else {
                    vec.clone()
                };
                vec.sort_by(display_sort);

                if vec.len() == 2 {
                    if let Ast::Num(v) = &vec[0] {
                        if *v == 1 {
                            s.push_str(&format!("{}", vec[1]));
                            current = 2;
                        } else if *v == -1 {
                            s.push_str(&format!("-{}", vec[1]));
                            current = 2;
                        }
                    } else if let Ast::Num(v) = &vec[1] {
                        if *v == 1 {
                            s.push_str(&format!("{}", vec[0]));
                            current = 2;
                        } else if *v == -1 {
                            s.push_str(&format!("-{}", vec[0]));
                            current = 2;
                        }
                    }
                }
                while current < len {
                    if current == 0 {
                        if let Ast::Add(_) = &vec[current] {
                            s.push_str(&format!("({})", vec[current]));
                        } else {
                            s.push_str(&format!("{}", vec[current]));
                        }
                    } else {
                        let el_s = if let Ast::Add(_) = &vec[current] {
                            format!("({})", vec[current])
                        } else {
                            format!("{}", vec[current])
                        };
                        s.push_str(&el_s);
                    }
                    if current < len - 1 {
                        s.push_str("*");
                    }
                    current += 1;
                }

                write!(f, "{}", s)
            }
            Ast::Symbol(name) | Ast::Const(name) => {
                write!(f, "{}", name)
            }
            Ast::Num(num) => {
                write!(f, "{}", num)
            }
            Ast::Pow(base, exp) => {
                let mut s = "".to_owned();
                if let Ast::Add(_) | Ast::Mul(_) = **base {
                    s.push_str(&format!("({})^", base));
                } else {
                    s.push_str(&format!("{}^", base));
                }
                if let Ast::Add(_) | Ast::Mul(_) = **exp {
                    s.push_str(&format!("({})", exp));
                } else {
                    s.push_str(&format!("{}", exp));
                }
                write!(f, "{}", s)
            }
            Ast::Func(name, args) => {
                let mut s = "".to_owned();
                for node in args {
                    s.push_str(&format!("{}, ", node));
                }
                s = s[0..s.len() - 2].to_string();

                write!(f, "{}({})", name, s)
            }
        }
    }
}

fn display_sort<N>(a: &Ast<N>, b: &Ast<N>) -> Ordering
where
    N: NumberType,
{
    match (a, b) {
        (Ast::Pow(_, a), Ast::Pow(_, b)) => match (*a.clone(), *b.clone()) {
            (Ast::Num(a), Ast::Num(b)) => {
                if a > b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Ast::Num(_), _) => Ordering::Less,
            (_, Ast::Num(_)) => Ordering::Greater,
            _ => Ordering::Equal,
        },
        _ => Ordering::Equal,
    }
}
