
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

fn instrs(body: &Body) -> Vec<TInstr> {
    let mut instructions: Vec<TInstr> = Vec::new();

    match body {
        Body::Return(val) => {
            let ret_val = expr_val(val, &mut instructions);
            instructions.push(TInstr::Return(ret_val));
        },
    };

    instructions
}

fn expr_val(expr: &Expr, instructions: &mut Vec<TInstr>) -> TVal {
    match expr {
        Expr::Constant(val) => TVal::Constant(*val),
        Expr::Unary(op, inner) => {
            let src = expr_val(inner, instructions);
            let dst_name = make_temp(instructions.len());
            let dst = TVal::Var(dst_name);
            let tacky_op = get_unary_op(op);
            instructions.push(TInstr::Unary(tacky_op, src, dst.clone()));

            dst
        }
    }
}

fn get_unary_op(op: &UnaryOp) -> TUnaryOp {
    match op {
        UnaryOp::Complement => TUnaryOp::Complement,
        UnaryOp::Negate => TUnaryOp::Negate,
    }
}

fn make_temp(ident: usize) -> String {
    format!("tmp.{}", ident)
}
