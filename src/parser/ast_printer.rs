
use crate::parser::ast::*;

pub fn print_ast(ast: FuncDef) {
    println!("Program(");
    print_function(ast, 1);
    println!(")");
}

fn print_function(ast: FuncDef, indent: u32) {
    match ast {
        FuncDef::Function(name, body) => {
            println!("{}{} <- Function(", level(indent), name);
            println!("{}body=", level(indent + 1));
            print_body(body, indent + 2);
            println!("{})", level(indent));
        },
    };
}

fn print_body(body: Vec<BlockItem>, indent: u32) {
    for item in body.iter() {
        print_block_item(item, indent);
    }
}

fn print_block_item(item: &BlockItem, indent: u32) {
    match item {
        BlockItem::S(stmt) => print_stmt(stmt, indent),
        BlockItem::D(decl) => print_decl(decl, indent),
    }
}

fn print_decl(decl: &Decl, indent: u32) {
    match decl {
        Decl::Declaration(ident, init) => {
            println!("{}{} <- Declaration(", level(indent), ident);
            print_init(init, indent + 1);
        }
    }
}

fn print_init(init: &Option<Expr>, indent: u32) {
    match init {
        Some(expr) => print_expr(expr, indent),
        None => println!("{}Uninitialized", level(indent)),
    }
}

fn print_stmt(stmt: &Stmt, indent: u32) {
    match stmt {
        Stmt::Return(expr) => {
            println!("{}Return(", level(indent));
            print_expr(expr, indent + 1);
            println!("{})", level(indent));
        },
        Stmt::Expression(expr) => {
            println!("{}Expression Statement(", level(indent));
            print_expr(expr, indent + 1);
            println!("{})", level(indent));
        },
        Stmt::Null => println!("{}Null Statement", level(indent)),
    }
}

fn print_expr(expr: &Expr, indent: u32) {
    match expr {
        Expr::Constant(val) => {
            println!("{}Constant({})", level(indent), val);
        },
        Expr::Unary(op, expr) => {
            print!("{}Unary ", level(indent));
            print_unary(op);
            print_expr(&*expr, indent + 1);
        },
        Expr::Binary(op, left, right) => {
            print!("{}Binary ", level(indent));
            print_binary(op);
            println!("{}Left(", level(indent + 1));
            print_expr(&*left, indent + 2);
            println!("{})Right(", level(indent + 1));
            print_expr(&*right, indent + 2);
            println!("{})", level(indent + 1));
        },
        Expr::Var(ident) => {
            println!("{}Var: {}", level(indent), ident);
        },
        Expr::Assignment(left, right) => {
            println!("{}Assignment:", level(indent));
            println!("{}Into(", level(indent + 1));
            print_expr(&*left, indent + 2);
            println!("{})From(", level(indent + 1));
            print_expr(&*right, indent + 2);
            println!("{})", level(indent + 1));
        }
    }
}

fn print_unary(op: &UnaryOp) {
    match op {
        UnaryOp::Negate => println!("-:"),
        UnaryOp::Complement => println!("~:"),
        UnaryOp::Not => println!("!:"),
    };
}

fn print_binary(op: &BinaryOp) {
    match op {
        BinaryOp::Add => println!("+:"),
        BinaryOp::Subtract => println!("-:"),
        BinaryOp::Multiply => println!("*:"),
        BinaryOp::Divide => println!("/:"),
        BinaryOp::Remainder => println!("%:"),
        BinaryOp::Equal => println!("==:"),
        BinaryOp::NotEqual => println!("!=:"),
        BinaryOp::LessThan => println!("<:"),
        BinaryOp::LessEqual => println!("<= :"),
        BinaryOp::GreatThan => println!(">:"),
        BinaryOp::GreatEqual => println!(">= :"),
        BinaryOp::And => println!("&&:"),
        BinaryOp::Or => println!("||:"),
        BinaryOp::Assign => (),
    };
}

fn level(num: u32) -> String {
    let mut indent = String::new();

    for _ in 0..num {
        indent.push(' ');
    }

    indent
}