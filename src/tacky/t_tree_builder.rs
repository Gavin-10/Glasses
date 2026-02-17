
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
            let dst_name = make_temp("temp", instructions.len());
            let dst = TVal::Var(dst_name);
            let tacky_op = get_unary_op(op);
            instructions.push(TInstr::Unary(tacky_op, src, dst.clone()));

            dst
        },
        Expr::Binary(BinaryOp::And, left, right) => {
            let result_name = make_temp("and_result", instructions.len());
            let result = TVal::Var(result_name);

            let v1 = expr_val(left, instructions);

            let false_name = make_temp("and_false", instructions.len());
            instructions.push(TInstr::JumpIfZero(v1, false_name.clone()));

            let v2 = expr_val(right, instructions);

            instructions.push(TInstr::JumpIfZero(v2, false_name.clone()));
            instructions.push(TInstr::Copy(TVal::Constant(1), result.clone()));

            let true_name = make_temp("and_true", instructions.len());
            instructions.push(TInstr::Jump(true_name.clone()));
            instructions.push(TInstr::Label(false_name));
            instructions.push(TInstr::Copy(TVal::Constant(0), result.clone()));
            instructions.push(TInstr::Label(true_name));

            result
        },
        Expr::Binary(BinaryOp::Or, left, right) => {
            let result_name = make_temp("or_result", instructions.len());
            let result = TVal::Var(result_name);

            let v1 = expr_val(left, instructions);

            let true_name = make_temp("or_true", instructions.len());
            instructions.push(TInstr::JumpIfNotZero(v1, true_name.clone()));

            let v2 = expr_val(right, instructions);

            instructions.push(TInstr::JumpIfNotZero(v2, true_name.clone()));
            instructions.push(TInstr::Copy(TVal::Constant(0), result.clone()));

            let false_name = make_temp("or_false", instructions.len());
            instructions.push(TInstr::Jump(false_name.clone()));
            instructions.push(TInstr::Label(true_name));
            instructions.push(TInstr::Copy(TVal::Constant(1), result.clone()));
            instructions.push(TInstr::Label(false_name));

            result
        }
        Expr::Binary(op, left, right) => {
            let v1 = expr_val(left, instructions);
            let v2 = expr_val(right, instructions);
            let dst_name = make_temp("temp", instructions.len());
            let dst = TVal::Var(dst_name);
            let tacky_op = get_binary_op(op);
            instructions.push(TInstr::Binary(tacky_op, v1, v2, dst.clone()));

            dst
        },
    }
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
