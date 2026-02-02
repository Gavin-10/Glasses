
use crate::utilities::error_handler::fmt_lexer_error;
use crate::lexer::lexer_structs::*;

pub fn lex(characters: Vec<u8>) -> Result<Vec<(Tkn, u32)>, String> {
    let mut lexer = Lxr::new(characters);

    while !lexer.is_at_end() {
        skip_white_space(&mut lexer);
        let token = create_token(&mut lexer)?;
        lexer.tokens.push((token, lexer.line));
    }

    Ok(lexer.tokens)
}

fn skip_white_space(lexer: &mut Lxr) {
    while !lexer.is_at_end() {
        match lexer.peek() {
            '\n' | '\r' => {
                lexer.line += 1;
                lexer.take();
            },
            ' ' | '\t' => {
                lexer.take();
            },
            _ => break,
        };
    }
}

fn create_token(lexer: &mut Lxr) -> Result<Tkn, String> {
    let current = lexer.take();
    match current {
        '(' => Ok(Tkn::LeftParen),
        ')' => Ok(Tkn::RightParen),
        '{' => Ok(Tkn::LeftBrace),
        '}' => Ok(Tkn::RightBrace),
        ';' => Ok(Tkn::Semicolon),
        '~' => Ok(Tkn::Tilde),

        '-' => {
            if lexer.peek() == '-' { 
                lexer.take();
                Ok(Tkn::Decrement)
            } else {
                Ok(Tkn::Subtract)
            }
        },

        _ => {
            if is_digit(current) {
                let mut num = String::from(current);
                while is_digit(lexer.peek()) { num.push(lexer.take()); }

                if is_alpha(lexer.peek()) { 
                    Err(fmt_lexer_error(lexer.line, "Invalid Identifier"))
                } else {
                    Ok(Tkn::Constant(num.parse().unwrap()))
                }
            } else if is_alpha(current) {
                let mut ident = String::from(current);
                while is_alpha_num(lexer.peek()) {
                    ident.push(lexer.take());
                }
                if KEYWORDS.contains(&ident.as_str()) {
                    Ok(Tkn::Key(ident))
                } else {
                    Ok(Tkn::Identifier(ident))
                }
            } else {
                Err(fmt_lexer_error(lexer.line, &format!("Unexpected Token: {}", current)))
            }
        } 
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <='9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_num(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}



