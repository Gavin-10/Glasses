
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
    tokens.consume(Tkn::Key(Keyword::Int), "Expected int");

    let expected_ident = tokens.next_token();
    let name = match expected_ident.0 {
        Tkn::Identifier(name) => name,
        _ => parser_error(expected_ident.1, "Expected function identifier"),
    };

    tokens.consume(Tkn::LeftParen, "Expected '('");
    tokens.consume(Tkn::Key(Keyword::Void), "Expected 'void'");
    tokens.consume(Tkn::RightParen, "Expected ')'");

    let body = body_decl(tokens);

    FuncDef::Function(name, body)
}

fn body_decl(tokens: &mut TokenQue) -> Vec<BlockItem> {
    let mut body: Vec<BlockItem> = Vec::new();

    tokens.consume(Tkn::LeftBrace, "Expected '{'");
    
    while tokens.peek_next_token().0 != Tkn::RightBrace {
        body.push(next_block_item(tokens));
    }

    tokens.consume(Tkn::RightBrace, "Expected '}'");

    body
}

fn next_block_item(tokens: &mut TokenQue) -> BlockItem {
    let current = tokens.peek_next_token();

    match current.0 {
        Tkn::Key(Keyword::Int) => {
            tokens.next();
            BlockItem::D(declaration(tokens))
        },
        _ => BlockItem::S(statement(tokens)),
    }
}

fn declaration(tokens: &mut TokenQue) -> Decl {
    let current = tokens.next_token();
    let ident;
    match current.0 {
        Tkn::Identifier(val) => ident = val,
        _ => parser_error(current.1, "Identifier Expected"),
    }

    let next = tokens.peek_next_token();
    let mut init = None;
    match next.0 {
        Tkn::Equal => {
            tokens.next();
            init = Some(expr(tokens, 0));
        },
        _ => (),
    }

    tokens.consume(Tkn::Semicolon, "Expected ';'");

    Decl::Declaration(ident, init)
}

fn statement(tokens: &mut TokenQue) -> Stmt {
    let current = tokens.peek_next_token();

    let res = match current.0 {
        Tkn::Semicolon => Stmt::Null,
        Tkn::Key(Keyword::Return) => {
            tokens.next();
            Stmt::Return(expr(tokens, 0))
        },
        _ => Stmt::Expression(expr(tokens, 0)),
    };
    tokens.consume(Tkn::Semicolon, "Expected ';'");
    
    res
}

fn expr(tokens: &mut TokenQue, min_prec: u32) -> Expr {
    let mut left = factor(tokens);
    let mut next_op = parse_binary_op(&tokens.peek_next_token());

    while next_op != None && precedence(&next_op.unwrap()) >= min_prec {
        let op = parse_binary_op(&tokens.next_token()).unwrap();
        match op {
            BinaryOp::Assign => {
                let right = expr(tokens, precedence(&op));
                left = Expr::Assignment(Box::from(left), Box::from(right));
            },
            _ => {
                let right = expr(tokens, precedence(&op) + 1);
                left = Expr::Binary(op, Box::from(left), Box::from(right));
            }
        }
        next_op = parse_binary_op(&tokens.peek_next_token());
    }

    left
}

fn factor(tokens: &mut TokenQue) -> Expr {
    
    let current = tokens.next_token();
    match current.0 {
        Tkn::Constant(value) => Expr::Constant(value),
        Tkn::Tilde | Tkn::Subtract | Tkn::Not => {
            let operator = parse_unary_op(&current);
            Expr::Unary(operator, Box::new(factor(tokens)))
        },
        Tkn::LeftParen => {
            let inner_expr = expr(tokens, 0);
            tokens.consume(Tkn::RightParen, "Expected ')'");
            inner_expr
        },
        Tkn::Identifier(val) => Expr::Var(val),

        _ => parser_error(current.1, "Expression Expected"),
    }
}

fn parse_unary_op(token: &(Tkn, u32)) -> UnaryOp {
    match token.0 {
        Tkn::Tilde => UnaryOp::Complement,
        Tkn::Subtract => UnaryOp::Negate,
        Tkn::Not => UnaryOp::Not,
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
        Tkn::EqualEqual => Some(BinaryOp::Equal),
        Tkn::Less => Some(BinaryOp::LessThan),
        Tkn::LessEqual => Some(BinaryOp::LessEqual),
        Tkn::Great => Some(BinaryOp::GreatThan),
        Tkn::GreatEqual => Some(BinaryOp::GreatEqual),
        Tkn::NotEqual => Some(BinaryOp::NotEqual),
        Tkn::And => Some(BinaryOp::And),
        Tkn::Or => Some(BinaryOp::Or),
        Tkn::Equal => Some(BinaryOp::Assign),
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
        BinaryOp::LessThan => 35,
        BinaryOp::LessEqual => 35,
        BinaryOp::GreatThan => 35,
        BinaryOp::GreatEqual => 35,
        BinaryOp::Equal => 30,
        BinaryOp::NotEqual => 30,
        BinaryOp::And => 10,
        BinaryOp::Or => 5,
        BinaryOp::Assign => 1,
    }
}