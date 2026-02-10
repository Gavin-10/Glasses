
pub static KEYWORDS: [&str; 3] = ["int", "void", "return"];

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

    //Words
    Key(String),
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
