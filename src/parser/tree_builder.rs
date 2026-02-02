
use crate::lexer::lexer_structs::*;
use crate::utilities::error_handler::parser_error;
use crate::parser::ast::*;

pub fn parse(tokens: Vec<(Tkn, u32)>) -> FuncDef {
    let mut token_que = TokenQue::new(tokens);
    let mut program: Option<FuncDef> = None;

    while token_que.len() != 0 {
        program = Some(fn_decl(&mut token_que));
    }

    program.unwrap()
}

fn fn_decl(tokens: &mut TokenQue) -> FuncDef {
    tokens.consume(Tkn::Key("int".to_string()), "Expected int");

    let expected_ident = tokens.next_token();
    let name = match expected_ident.0 {
        Tkn::Identifier(name) => name,
        _ => parser_error(expected_ident.1, "Expected function identifier"),
    };

    tokens.consume(Tkn::LeftParen, "Expected '('");
    tokens.consume(Tkn::Key("void".to_string()), "Expected 'void'");
    tokens.consume(Tkn::RightParen, "Expected ')'");

    let body = body_decl(tokens);

    FuncDef::Function(name, body)
}

fn body_decl(tokens: &mut TokenQue) -> Body {
    tokens.consume(Tkn::LeftBrace, "Expected '{'");
    tokens.consume(Tkn::Key("return".to_string()), "Expected 'return'");

    let expression = expr(tokens);

    tokens.consume(Tkn::Semicolon, "Expected ';'");
    tokens.consume(Tkn::RightBrace, "Expected '}'");

    Body::Return(expression)
}

fn expr(tokens: &mut TokenQue) -> Expr {
    
    let current = tokens.next_token();
    match current.0 {
        Tkn::Constant(value) => Expr::Constant(value),
        Tkn::Tilde | Tkn::Subtract => {
            let operator = parse_unary_op(&current);
            Expr::Unary(operator, Box::new(expr(tokens)))
        },
        Tkn::LeftParen => {
            let inner_expr = expr(tokens);
            tokens.consume(Tkn::RightParen, "Expected ')'");
            inner_expr
        },

        _ => parser_error(current.1, "Expression Expected"),
    }
}

fn parse_unary_op(token: &(Tkn, u32)) -> UnaryOp {
    match token.0 {
        Tkn::Tilde => UnaryOp::Complement,
        Tkn::Subtract => UnaryOp::Negate,
        _ => parser_error(token.1, "Unary Operator Expected"),
    }
}