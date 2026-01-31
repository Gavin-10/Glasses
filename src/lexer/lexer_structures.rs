
pub static KEYWORDS: [&str; 3] = ["int", "void", "return"];

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    //Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,

    //Words
    Key(String),
    Ident(String),
    Const(i32)
}

pub struct Lexer {
    pub tokens: Vec<(Token, u32)>,
    pub line: u32,
    characters: Vec<u8>,
    current: usize,
}

impl Lexer {
    pub fn new(characters: Vec<u8>) -> Self {
        Self { characters: characters, current: 0, tokens: Vec::new(), line: 1 }
    }

    pub fn peek(&self) -> char {
        self.characters[self.current] as char
    }

    pub fn take(&mut self) -> char {
        self.current += 1;
        self.characters[self.current - 1] as char
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.characters.len() - 1 || self.peek() == '\0'
    }
}
