
use std::collections::VecDeque;
use crate::lexer::lexer_structs::*;
use crate::utilities::error_handler::*;

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug)]
pub enum Expr {
    Constant(i32),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Body {
    Return(Expr),
}

#[derive(Debug)]
pub enum FuncDef {
    Function(String, Body),
}

pub struct TokenQue {
    tokens: VecDeque<(Tkn, u32)>
}

impl TokenQue {
    pub fn new(tokens: Vec<(Tkn, u32)>) -> Self {
        Self { tokens: VecDeque::from(tokens) }
    }

    pub fn consume(&mut self, expected: Tkn, msg: &str) {
        let front = self.peek_next_token();

        if expected == front.0 {
            let _ = self.next_token();
        } else {
            parser_error(front.1, msg);
        }
    }

    pub fn next_token(&mut self) -> (Tkn, u32) {
        let token = match self.tokens.pop_front() {
            Some(res) => res,
            None => parser_error_no_line("Expected token but recieved none"),
        };

        token
    }

    pub fn peek_next_token(&mut self) -> &(Tkn, u32) {
        match self.tokens.front() {
            Some(res) => res,
            None => parser_error_no_line("Expected token but recieved none"),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}