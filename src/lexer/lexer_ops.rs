
use crate::utilities::error_handler::fmt_lexer_error;
use crate::lexer::lexer_structs::*;

pub fn lex(characters: Vec<u8>) -> Result<Vec<(Tkn, u32)>, String> {
    let mut lexer = Lxr::new(characters);
    while !lexer.is_at_end() {
        skip_white_space(&mut lexer)?;
        let token = create_token(&mut lexer)?;
        lexer.tokens.push((token, lexer.line));
    }

    Ok(lexer.tokens)
}

fn skip_white_space(lexer: &mut Lxr) -> Result<(), String> {
    while !lexer.is_at_end() {
        match lexer.peek()? {
            '\n' | '\r' => {
                lexer.line += 1;
                lexer.take()?;
            },
            ' ' | '\t' => {
                lexer.take()?;
            },
            _ => break,
        };
    }

    Ok(())
}

fn create_token(lexer: &mut Lxr) -> Result<Tkn, String> {
    let current = lexer.take()?;
    match current {
        '(' => Ok(Tkn::LeftParen),
        ')' => Ok(Tkn::RightParen),
        '{' => Ok(Tkn::LeftBrace),
        '}' => Ok(Tkn::RightBrace),
        ';' => Ok(Tkn::Semicolon),
        '~' => Ok(Tkn::Tilde),
        '/' => Ok(Tkn::Slash),
        '*' => Ok(Tkn::Star),
        '%' => Ok(Tkn::Mod),

        '-' => match_dbl(Tkn::Decrement, Tkn::Subtract, '-', lexer),
        '+' => match_dbl(Tkn::Increment, Tkn::Plus, '+', lexer),
        '!' => match_dbl(Tkn::NotEqual, Tkn::Not, '=', lexer),
        '=' => match_dbl(Tkn::EqualEqual, Tkn::Equal, '=', lexer),
        '<' => match_dbl(Tkn::LessEqual, Tkn::Less, '=', lexer),
        '>' => match_dbl(Tkn::GreatEqual, Tkn::Great, '=', lexer),
        '&' => match_dbl(Tkn::And, Tkn::BAnd, '&', lexer),
        '|' => match_dbl(Tkn::Or, Tkn::BOr, '|', lexer),

        _ => {
            if is_digit(current) {
                let mut num = String::from(current);
                while is_digit(lexer.peek()?) { num.push(lexer.take()?); }

                if is_alpha(lexer.peek()?) { 
                    Err(fmt_lexer_error(lexer.line, "Invalid Identifier"))
                } else {
                    Ok(Tkn::Constant(num.parse().unwrap()))
                }
            } else if is_alpha(current) {
                let mut ident = String::from(current);
                while is_alpha_num(lexer.peek()?) {
                    ident.push(lexer.take()?);
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

fn match_dbl(a: Tkn, b: Tkn, e: char, lexer: &mut Lxr) -> Result<Tkn, String> {
    if lexer.peek()? == e {
        lexer.take()?;
        Ok(a)
    } else {
        Ok(b)
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