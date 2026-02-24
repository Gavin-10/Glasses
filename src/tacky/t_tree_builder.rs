
use crate::tacky::t_ast::*;
use crate::parser::ast::*;

pub fn gen_tacky(ast: FuncDef) -> TFuncDef {
    let tacky_tree = function_decl(&ast);

    tacky_tree
}

fn function_decl(ast: &FuncDef) -> TFuncDef {
    match ast {
        FuncDef::Function(ident, body) => TFuncDef::Function(ident.to_string(), instrs(&body)),
    }
}

fn instrs(body: &Vec<BlockItem>) -> Vec<TInstr> {
    let mut instructions: Vec<TInstr> = Vec::new();

    for item in body.iter() {
        block_item(item, &mut instructions);
    }

    instructions.push(TInstr::Return(TVal::Constant(0)));
    
    instructions
}

fn block_item(item: &BlockItem, instructions: &mut Vec<TInstr>) {
    match item {
        BlockItem::S(stmt) => stmt_val(stmt, instructions),
        BlockItem::D(decl) => decl_val(decl, instructions),
    }
}

fn decl_val(decl: &Decl, instructions: &mut Vec<TInstr>) {
    match decl {
        Decl::Declaration(var, Some(expr)) => {
            let res = expr_val(expr, instructions);
            instructions.push(TInstr::Copy(res, TVal::Var(var.to_string())));
        },
        _ => (),
    }
}

fn stmt_val(stmt: &Stmt, instructions: &mut Vec<TInstr>) {
    match stmt {
        Stmt::Return(val) => {
            let ret_val = expr_val(val, instructions);
            instructions.push(TInstr::Return(ret_val));
        },
        Stmt::Expression(expr) => {
            let _ = expr_val(expr, instructions);
        },
        _ => (),
    }
}

fn expr_val(expr: &Expr, instructions: &mut Vec<TInstr>) -> TVal {
    match expr {
        Expr::Constant(val) => TVal::Constant(*val),
        Expr::Unary(op, inner) => {
            let src = expr_val(inner, instructions);
            let dst_name = make_temp("temp", instructions.len());
            let dst = TVal::Var(dst_name);
            let tacky_op = get_unary_op(op);
            instructions.push(TInstr::Unary(tacky_op, src, dst.clone()));

            dst
        },
        Expr::Binary(op, left, right) => binary(&op, left, right, instructions),
        Expr::Var(v) => TVal::Var(v.clone()),
        Expr::Assignment(var, right) => {
            let mut v = String::new();
            if let Expr::Var(name) = &**var {
                v = name.clone();
            }
            let res = expr_val(right, instructions);
            instructions.push(TInstr::Copy(res, TVal::Var(v.clone())));

            TVal::Var(v)
        },
    }
}

fn binary(op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>, instructions: &mut Vec<TInstr>) -> TVal {
    match op {
        BinaryOp::And | BinaryOp::Or => and_or(op, left, right, instructions),
        _ => binary_normal(op, left, right, instructions),
    }
}

fn and_or(op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>, instructions: &mut Vec<TInstr>) -> TVal {
    let result_name = make_temp("and_or_result", instructions.len());
    let result = TVal::Var(result_name);
    let short_cond;
    let final_val;

    match op {
        BinaryOp::And => {
            let v1 = expr_val(left, instructions);
            short_cond = make_temp("short_cond", instructions.len());
            instructions.push(TInstr::JumpIfZero(v1, short_cond.clone()));
            let v2 = expr_val(right, instructions);
            instructions.push(TInstr::JumpIfZero(v2,  short_cond.clone()));
            instructions.push(TInstr::Copy(TVal::Constant(1), result.clone()));
            final_val = 0;
        },
        BinaryOp::Or => {
            let v1 = expr_val(left, instructions);
            short_cond = make_temp("short_cond", instructions.len());
            instructions.push(TInstr::JumpIfNotZero(v1, short_cond.clone()));
            let v2 = expr_val(right, instructions);
            instructions.push(TInstr::JumpIfNotZero(v2, short_cond.clone()));
            instructions.push(TInstr::Copy(TVal::Constant(0), result.clone()));
            final_val = 1;
        },
        _ => panic!(), //Unreachable
    }

    let jmp_cond = make_temp("jmp_cond", instructions.len());
    instructions.push(TInstr::Jump(jmp_cond.clone()));
    instructions.push(TInstr::Label(short_cond));
    instructions.push(TInstr::Copy(TVal::Constant(final_val), result.clone()));
    instructions.push(TInstr::Label(jmp_cond));

    result
}

fn binary_normal(op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>, instructions: &mut Vec<TInstr>) -> TVal {
    let v1 = expr_val(left, instructions);
    let v2 = expr_val(right, instructions);
    let dst_name = make_temp("temp", instructions.len());
    let dst = TVal::Var(dst_name);
    let tacky_op = get_binary_op(op);
    instructions.push(TInstr::Binary(tacky_op, v1, v2, dst.clone()));

    dst
}

fn get_unary_op(op: &UnaryOp) -> TUnaryOp {
    match op {
        UnaryOp::Complement => TUnaryOp::Complement,
        UnaryOp::Negate => TUnaryOp::Negate,
        UnaryOp::Not => TUnaryOp::Not,
    }
}

fn get_binary_op(op: &BinaryOp) -> TBinaryOp {
    match op {
        BinaryOp::Add => TBinaryOp::Add,
        BinaryOp::Subtract => TBinaryOp::Subtract,
        BinaryOp::Multiply => TBinaryOp::Multiply,
        BinaryOp::Divide => TBinaryOp::Divide,
        BinaryOp::Remainder => TBinaryOp::Remainder,
        BinaryOp::Equal => TBinaryOp::Equal,
        BinaryOp::NotEqual => TBinaryOp::NotEqual,
        BinaryOp::LessThan => TBinaryOp::LessThan,
        BinaryOp::LessEqual => TBinaryOp::LessEqual,
        BinaryOp::GreatThan => TBinaryOp::GreatThan,
        BinaryOp::GreatEqual => TBinaryOp::GreatEqual,
        _ => panic!("Unexpected binary operator"),
    }
}

fn make_temp(start: &str, ident: usize) -> String {
    format!("{}.{}", start, ident)
}
