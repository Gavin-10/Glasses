
use crate::parser::ast::*;

pub fn print_ast(ast: FuncDef) {
    println!("Program(");
    print_function(ast, 1);
    println!(")");
}

fn print_function(ast: FuncDef, indent: u32) {
    match ast {
        FuncDef::Function(name, body) => {
            println!("{}Function(", level(indent));
            println!("{}name=\"{}\",", level(indent + 1), name);
            print!("{}body=", level(indent + 1));
            print_body(body, indent + 1);
            println!("{})", level(indent));
        },
    };
}

fn print_body(body: Body, indent: u32) {
    match body {
        Body::Return(expr) => {
            println!("Return(");
            print_expr(expr, indent + 1);
            println!("{})", level(indent));
        },
    };
}

fn print_expr(expr: Expr, indent: u32) {
    match expr {
        Expr::Constant(val) => {
            println!("{}Constant({})", level(indent), val);
        },
        Expr::Unary(op, expr) => {
            print!("{}Unary ", level(indent));
            print_unary(op);
            print_expr(*expr, indent + 1);
        },
        Expr::Binary(op, left, right) => {
            print!("{}Binary ", level(indent));
            print_binary(op);
            println!("{}Left(", level(indent + 1));
            print_expr(*left, indent + 2);
            println!("{})Right(", level(indent + 1));
            print_expr(*right, indent + 2);
            println!("{})", level(indent + 1));
        },
    }
}

fn print_unary(op: UnaryOp) {
    match op {
        UnaryOp::Negate => println!("-:"),
        UnaryOp::Complement => println!("~:"),
    };
}

fn print_binary(op: BinaryOp) {
    match op {
        BinaryOp::Add => println!("+:"),
        BinaryOp::Subtract => println!("-:"),
        BinaryOp::Multiply => println!("*:"),
        BinaryOp::Divide => println!("/:"),
        BinaryOp::Remainder => println!("%:"),
    };
}

fn level(num: u32) -> String {
    let mut indent = String::new();

    for _ in 0..num {
        indent.push(' ');
    }

    indent
}