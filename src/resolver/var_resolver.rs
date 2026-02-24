
use std::collections::HashMap;

use crate::parser::ast::*;
use crate::utilities::error_handler::resolver_error;

pub fn resolve_vars(ast: &mut FuncDef) {
    let mut var_map: HashMap<String, String> = HashMap::new();

    match ast {
        FuncDef::Function(_, items) => {
            for item in items.iter_mut() {
                *item = resolve_item(&item, &mut var_map);
            }
        }
    }
}

fn resolve_item(item: &BlockItem, var_map: &mut HashMap<String, String>) -> BlockItem {
    match item {
        BlockItem::D(decl) => BlockItem::D(resolve_decl(decl, var_map)),
        BlockItem::S(stmt) => BlockItem::S(resolve_stmt(stmt, var_map)),
    }
}

fn resolve_decl(decl: &Decl, var_map: &mut HashMap<String, String>) -> Decl {
    match decl {
        Decl::Declaration(name, init) => {
            if var_map.contains_key(name) {
                resolver_error(format!("{} is a duplicate variable declaration", name).as_str());
            }

            let new_name = unique_name(&name, var_map.len());
            var_map.insert(name.clone(), new_name.clone());

            let mut resolved_init = None;

            match init {
                Some(expr) => {
                    resolved_init = Some(resolve_expr(expr, var_map));
                },
                None => (),
            }

            Decl::Declaration(new_name, resolved_init)
        }
    }
}

fn resolve_stmt(stmt: &Stmt, var_map: &mut HashMap<String, String>) -> Stmt {
    match stmt {
        Stmt::Return(expr) => Stmt::Return(resolve_expr(expr, var_map)),
        Stmt::Expression(expr) => Stmt::Expression(resolve_expr(expr, var_map)),
        Stmt::Null => Stmt::Null,
    }
}

fn resolve_expr(expr: &Expr, var_map: &mut HashMap<String, String>) -> Expr {
    match expr {
        Expr::Assignment(left, right) => resolve_assignment(left, right, var_map),
        Expr::Var(v) => resolve_var(v, var_map),
        Expr::Binary(op, left, right) => Expr::Binary(op.clone(), 
            Box::new(resolve_expr(left, var_map)), 
            Box::new(resolve_expr(right, var_map))),
        Expr::Unary(op, oprnd) => Expr::Unary(op.clone(), Box::new(resolve_expr(oprnd, var_map))),
        Expr::Constant(_) => expr.clone(),
    }
}

fn resolve_assignment(left: &Expr, right: &Expr, var_map: &mut HashMap<String, String>) -> Expr {
    let to_assign = match left {
        Expr::Var(_) => left.clone(),
        _ => resolver_error("Invalid lvalue"),
    };

    Expr::Assignment(Box::new(resolve_expr(&to_assign, var_map)), Box::new(resolve_expr(right, var_map)))
}

fn resolve_var(var: &String, var_map: &mut HashMap<String, String>) -> Expr {
    if var_map.contains_key(var) {
        Expr::Var(var_map.get(var).unwrap().to_string())
    } else {
        resolver_error(format!("{} is an undeclared variable", var).as_str())
    }
}

fn unique_name(name: &String, postfix: usize) -> String {
    String::from(name.to_owned() + "." + format!("{}", postfix).as_str())
}