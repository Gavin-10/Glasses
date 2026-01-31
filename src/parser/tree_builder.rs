
use crate::lexer::lexer_structures::*;
use crate::utilities::error_handler::parser_error;
use crate::parser::ast::*;

pub fn parse(tokens: Vec<(Token, u32)>) -> FunctionDef {
    let mut token_que = TokenQue::new(tokens);
    let mut program: Option<FunctionDef> = None;

    while token_que.len() != 0 {
        program = Some(fn_decl(&mut token_que));
    }

    program.unwrap()
}

fn fn_decl(tokens: &mut TokenQue) -> FunctionDef {
    tokens.consume(Token::Key("int".to_string()), "Expected int");

    let expected_ident = tokens.next_token();
    let name = match expected_ident.0 {
        Token::Ident(name) => name,
        _ => parser_error(expected_ident.1, "Expected function identifier"),
    };

    tokens.consume(Token::LeftParen, "Expected '('");
    tokens.consume(Token::Key("void".to_string()), "Expected 'void'");
    tokens.consume(Token::RightParen, "Expected ')'");

    let body = body_decl(tokens);

    FunctionDef::Function(name, body)
}

fn body_decl(tokens: &mut TokenQue) -> Body {
    tokens.consume(Token::LeftBrace, "Expected '{'");
    tokens.consume(Token::Key("return".to_string()), "Expected 'return'");

    let expression = expr(tokens);

    tokens.consume(Token::Semicolon, "Expected ';'");
    tokens.consume(Token::RightBrace, "Expected '}'");

    Body::Return(expression)
}

fn expr(tokens: &mut TokenQue) -> Expr {
    
    let current = tokens.next_token();
    let val = match current.0 {
        Token::Const(value) => value,
        _ => parser_error(current.1, "Expected constant return value"),
    };

    Expr::Constant(val)
}