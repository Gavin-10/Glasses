
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

    let expression = expr(tokens, 0);

    tokens.consume(Tkn::Semicolon, "Expected ';'");
    tokens.consume(Tkn::RightBrace, "Expected '}'");

    Body::Return(expression)
}

fn factor(tokens: &mut TokenQue) -> Expr {
    
    let current = tokens.next_token();
    match current.0 {
        Tkn::Constant(value) => Expr::Constant(value),
        Tkn::Tilde | Tkn::Subtract => {
            let operator = parse_unary_op(&current);
            Expr::Unary(operator, Box::new(factor(tokens)))
        },
        Tkn::LeftParen => {
            let inner_expr = expr(tokens, 0);
            tokens.consume(Tkn::RightParen, "Expected ')'");
            inner_expr
        },

        _ => parser_error(current.1, "Expression Expected"),
    }
}

fn expr(tokens: &mut TokenQue, min_prec: u32) -> Expr {
    let mut left = factor(tokens);
    let mut next_op = parse_binary_op(&tokens.peek_next_token());

    while next_op != None && precedence(&next_op.unwrap()) >= min_prec {
        let op = parse_binary_op(&tokens.next_token()).unwrap();
        let right = expr(tokens, precedence(&op) + 1);
        left = Expr::Binary(op, Box::from(left), Box::from(right));
        next_op = parse_binary_op(&tokens.peek_next_token());
    }

    left
}

fn parse_unary_op(token: &(Tkn, u32)) -> UnaryOp {
    match token.0 {
        Tkn::Tilde => UnaryOp::Complement,
        Tkn::Subtract => UnaryOp::Negate,
        _ => parser_error(token.1, "Unary Operator Expected"),
    }
}

fn parse_binary_op(token: &(Tkn, u32)) -> Option<BinaryOp> {
    match token.0 {
        Tkn::Subtract => Some(BinaryOp::Subtract),
        Tkn::Plus => Some(BinaryOp::Add),
        Tkn::Star => Some(BinaryOp::Multiply),
        Tkn::Slash => Some(BinaryOp::Divide),
        Tkn::Mod => Some(BinaryOp::Remainder),
        _ => None,
    }
}

fn precedence(op: &BinaryOp) -> u32 {
    match op {
        BinaryOp::Multiply => 50,
        BinaryOp::Divide => 50,
        BinaryOp::Remainder => 50,
        BinaryOp::Add => 45,
        BinaryOp::Subtract => 45,
    }
}