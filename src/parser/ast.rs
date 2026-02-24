
use std::collections::VecDeque;
use crate::lexer::lexer_structs::*;
use crate::utilities::error_handler::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum UnaryOp {
    Complement,
    Negate,
    Not,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreatThan,
    GreatEqual,
    Assign,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Expr {
    Constant(i32),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Var(String),
    Assignment(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Expression(Expr),
    Null,
}

#[derive(Debug)]
pub enum Decl {
    Declaration(String, Option<Expr>),
}

#[derive(Debug)]
pub enum BlockItem {
    S(Stmt),
    D(Decl),
}

#[derive(Debug)]
pub enum FuncDef {
    Function(String, Vec<BlockItem>),
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

    pub fn next(&mut self) {
        match self.tokens.pop_front() {
            Some(_) => (),
            None => parser_error_no_line("Expected token but recieved none"),
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