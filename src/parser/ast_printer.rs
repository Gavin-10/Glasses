
use crate::parser::ast::*;

pub fn print_ast(ast: FunctionDef) {
    println!("Program(");
    print_function(ast, 1);
    println!(")");
}

fn print_function(ast: FunctionDef, indent: u32) {
    match ast {
        FunctionDef::Function(name, body) => {
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
        }
    }
}

fn level(num: u32) -> String {
    let mut indent = String::new();

    for _ in 0..num {
        indent.push(' ');
    }

    indent
}