use crate::types::{prim_num::PrimNum, Operator, Token};

pub struct Lexer {
    term: Vec<char>,
    length: usize,
    idx: usize,
}

impl Lexer {
    pub fn new(term: &str) -> Lexer {
        let mut t = String::from(term);
        t.retain(|c| !c.is_whitespace());
        Lexer {
            term: t.chars().collect(),
            length: t.chars().count(),
            idx: 0,
        }
    }

    pub fn into_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn peek(&self) -> Option<char> {
        if self.idx < self.length {
            Some(self.term[self.idx])
        } else {
            None
        }
    }

    fn forward(&mut self) -> Option<char> {
        if self.idx < self.length {
            self.idx = self.idx + 1;
            Some(self.term[self.idx - 1])
        } else {
            None
        }
    }

    fn get_number(&mut self) -> Option<Token> {
        let next_char = self.peek()?;
        if !next_char.is_numeric() && next_char != '.' {
            return None;
        }

        let mut number = String::from("");
        number.push(self.forward()?);
        while let Some(next_char) = self.peek() {
            if !next_char.is_numeric() && next_char != '.' {
                break;
            }
            number.push(self.forward()?);
        }
        if number.contains(".") {
            Some(Token::Num(PrimNum::Float(number.parse::<f64>().unwrap())))
        } else {
            Some(Token::Num(PrimNum::Int(number.parse::<i128>().unwrap())))
        }
    }

    fn get_operator(&mut self) -> Option<Token> {
        let next_char = self.peek()?;
        match next_char {
            '+' => {
                self.forward();
                Some(Token::Op(Operator::Add))
            }
            '-' => {
                self.forward();
                Some(Token::Op(Operator::Sub))
            }
            '*' => {
                self.forward();
                Some(Token::Op(Operator::Mul))
            }
            '/' => {
                self.forward();
                Some(Token::Op(Operator::Div))
            }
            '^' => {
                self.forward();
                Some(Token::Op(Operator::Pow))
            }
            '(' => {
                self.forward();
                Some(Token::LParen)
            }
            ')' => {
                self.forward();
                Some(Token::RParen)
            }
            ',' => {
                self.forward();
                Some(Token::Comma)
            }
            _ => None,
        }
    }

    fn get_var_func(&mut self) -> Option<Token> {
        let next_char = self.peek()?;
        if !next_char.is_alphabetic() {
            return None;
        }

        let mut var_func = String::from("");
        var_func.push(self.forward()?);
        while let Some(next_char) = self.peek() {
            if !next_char.is_alphanumeric() {
                break;
            }
            var_func.push(self.forward()?);
        }

        match self.peek() {
            Some(next_char) => {
                if next_char == '(' {
                    Some(Token::Func(var_func))
                } else {
                    Some(Token::Var(var_func))
                }
            }
            None => Some(Token::Var(var_func)),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.idx >= self.length {
            None
        } else if let Some(i) = self.get_number() {
            Some(i)
        } else if let Some(i) = self.get_operator() {
            Some(i)
        } else if let Some(i) = self.get_var_func() {
            Some(i)
        } else {
            None
        }
    }
}
