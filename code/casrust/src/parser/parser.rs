use crate::evaluator::EvalFn;
use crate::types::{ast::Ast, ast::AstError, prim_num::PrimNum, Operator, Token};
use std::iter::Peekable;
use std::slice::Iter;

/// Tries to process tokens to return an modified abstract syntax tree.
pub struct Parser<'a> {
    /// The evaluations functions which are applied.
    pub evaler: &'a EvalFn<PrimNum>,
}

impl<'a> Parser<'a> {
    /// Tries to process tokens to return an modified abstract syntax tree.
    pub fn parse(&self, tokens: &Vec<Token>) -> Result<Ast<PrimNum>, AstError> {
        let mut iter = tokens.iter().peekable();
        self.parse_add(&mut iter)
    }

    fn parse_add(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Ast<PrimNum>, AstError> {
        let mut t = Ast::Add(vec![]);
        t = t + self.parse_mul(iter)?;
        while self.cmp_iter_peek(iter.peek(), Token::Op(Operator::Add))
            || self.cmp_iter_peek(iter.peek(), Token::Op(Operator::Sub))
        {
            match iter.peek() {
                Some(Token::Op(Operator::Add)) => {
                    iter.next();
                    t = t + self.parse_mul(iter)?;
                }
                Some(Token::Op(Operator::Sub)) => {
                    iter.next();
                    let next_term = self.parse_mul(iter)?;
                    match next_term {
                        Ast::Num(number) => {
                            t = t + Ast::Num(number * -1);
                        }
                        Ast::Add(v) => {
                            for node in v {
                                t = t + (node * Ast::Num(PrimNum::Int(-1)));
                            }
                        }
                        _ => {
                            t = t + Ast::Mul(vec![Ast::Num(PrimNum::Int(-1)), next_term]);
                        }
                    }
                }
                _ => {}
            }
        }

        t.shorten().sort();
        Ok(t)
    }

    fn parse_mul(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Ast<PrimNum>, AstError> {
        let mut t = Ast::Mul(vec![]);
        t = t * self.parse_pow(iter)?;
        while self.cmp_iter_peek(iter.peek(), Token::Op(Operator::Mul))
            || self.cmp_iter_peek(iter.peek(), Token::Op(Operator::Div))
            || self.cmp_iter_peek(iter.peek(), Token::Op(Operator::Pow))
        {
            match iter.peek() {
                Some(Token::Op(Operator::Mul)) => {
                    iter.next();
                    t = t * self.parse_pow(iter)?;
                }
                Some(Token::Op(Operator::Div)) => {
                    iter.next();
                    t = t * Ast::Pow(
                        Box::new(self.parse_pow(iter)?),
                        Box::new(Ast::Num(PrimNum::Int(-1))),
                    );
                }
                _ => {}
            }
        }

        t.shorten().sort();
        Ok(t)
    }

    fn parse_pow(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Ast<PrimNum>, AstError> {
        let mut ast = self.parse_unary(iter)?;
        while self.cmp_iter_peek(iter.peek(), Token::Op(Operator::Pow)) {
            match iter.peek() {
                Some(Token::Op(Operator::Pow)) => {
                    iter.next();
                    ast = Ast::Pow(Box::new(ast), Box::new(self.parse_unary(iter)?));
                }
                _ => {}
            }
        }

        Ok(ast)
    }

    fn parse_unary(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Ast<PrimNum>, AstError> {
        match iter.peek() {
            Some(Token::Op(Operator::Sub)) => Ok(Ast::Num(PrimNum::Int(0))),
            Some(Token::Op(Operator::Add)) => Ok(Ast::Num(PrimNum::Int(0))),
            _ => self.parse_primary(iter),
        }
    }

    fn parse_primary(&self, iter: &mut Peekable<Iter<Token>>) -> Result<Ast<PrimNum>, AstError> {
        match iter.peek() {
            None => Err(AstError),
            Some(Token::Var(name)) => {
                iter.next();
                if self.evaler.consts.contains_key(name) {
                    Ok(Ast::Const(name.clone()))
                } else {
                    Ok(Ast::Symbol(name.clone()))
                }
            }
            Some(Token::Func(name)) => {
                iter.next();
                Ok(Ast::Func(name.clone(), self.parse_function(iter)?))
            }
            Some(Token::Num(number)) => {
                iter.next();
                Ok(Ast::Num(number.clone()))
            }
            Some(Token::LParen) => {
                iter.next();
                let ast = self.parse_add(iter);
                if !self.cmp_iter_token(iter.next(), Token::RParen) {
                    return Err(AstError);
                }

                ast
            }
            _ => Err(AstError),
        }
    }

    fn parse_function(
        &self,
        iter: &mut Peekable<Iter<Token>>,
    ) -> Result<Vec<Ast<PrimNum>>, AstError> {
        if !self.cmp_iter_token(iter.next(), Token::LParen) {
            return Err(AstError);
        }
        let args: Vec<Ast<PrimNum>> = if !self.cmp_iter_peek(iter.peek(), Token::RParen) {
            self.parse_arguments(iter)?
        } else {
            vec![]
        };
        if !self.cmp_iter_token(iter.next(), Token::RParen) {
            return Err(AstError);
        }

        Ok(args)
    }

    fn parse_arguments(
        &self,
        iter: &mut Peekable<Iter<Token>>,
    ) -> Result<Vec<Ast<PrimNum>>, AstError> {
        let mut args = vec![];
        loop {
            let ast = self.parse_add(iter)?;
            args.push(ast);
            if !self.cmp_iter_peek(iter.peek(), Token::Comma) {
                break;
            }
            iter.next();
        }

        Ok(args)
    }

    fn cmp_iter_token(&self, iter_token: Option<&Token>, token: Token) -> bool {
        match iter_token {
            None => false,
            _ => *iter_token.unwrap() == token,
        }
    }

    fn cmp_iter_peek(&self, iter_token: Option<&&Token>, token: Token) -> bool {
        match iter_token {
            None => false,
            _ => **iter_token.unwrap() == token,
        }
    }
}
