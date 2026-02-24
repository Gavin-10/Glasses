
use crate::utilities::error_handler::fmt_lexer_error;

pub static KEYWORDS: [&str; 3] = ["int", "void", "return"];

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Tkn {
    //Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Tilde,

    //Single-Double Tokens
    Subtract,
    Decrement,
    Star,
    Slash,
    Plus,
    Increment,
    Mod,
    Not,
    NotEqual,
    BAnd,
    And,
    BOr,
    Or,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Great,
    GreatEqual,

    //Words
    Key(Keyword),
    Identifier(String),
    Constant(i32)
}

pub struct Lxr {
    pub tokens: Vec<(Tkn, u32)>,
    pub line: u32,
    characters: Vec<u8>,
    current: usize,
}

impl Lxr {
    pub fn new(characters: Vec<u8>) -> Self {
        Self { characters: characters, current: 0, tokens: Vec::new(), line: 1 }
    }

    pub fn peek(&self) -> Result<char, String> {
        if !(self.is_at_end()) {
            Ok(self.characters[self.current] as char)
        } else {
            Err(fmt_lexer_error(self.line, "No token to peek"))
        }
    }

    pub fn take(&mut self) -> Result<char, String> {
        if !(self.is_at_end()) {
            self.current += 1;
            Ok(self.characters[self.current - 1] as char)
        } else {
            Err(fmt_lexer_error(self.line, "No token to take"))
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.characters.len() - 1 || self.characters[self.current] as char == '\0'
    }
}
