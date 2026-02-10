
use crate::code_gen::a_ast::*;
use crate::code_gen::rep_pseudo_reg::rep_pseudo_regs;
use crate::code_gen::rep_invalid_instrs::rep_instrs;
use crate::tacky::t_ast::*;

pub fn gen_code(ast: TFuncDef) -> AssemFuncDef {
    let mut assembly_tree = function_decl(&ast);
    let stack_size = rep_pseudo_regs(&mut assembly_tree);
    rep_instrs(&mut assembly_tree, stack_size);

    assembly_tree
}

fn function_decl(ast: &TFuncDef) -> AssemFuncDef {
    match ast {
        TFuncDef::Function(identifier, body) => AssemFuncDef::Function(identifier.to_string(), instructions(&body)),
    }
}

fn instructions(body: &Vec<TInstr>) -> Vec<AInstr> {
    let mut instructions: Vec<AInstr> = Vec::new();

    for tinstr in body.iter() {
        instruction(&tinstr, &mut instructions);
    }

    instructions
}

fn instruction(instr: &TInstr, ainstrs: &mut Vec<AInstr>) {
    match instr {
        TInstr::Return(val) => {
            ainstrs.push(AInstr::Mov(operand(&val), AOprnd::Reg(AReg::AX)));
            ainstrs.push(AInstr::Ret);
        },
        TInstr::Unary(op, src, dst) => {
            ainstrs.push(AInstr::Mov(operand(&src), operand(&dst)));
            ainstrs.push(AInstr::Unary(unary_op(&op), operand(&dst)));
        },
        TInstr::Binary(TBinaryOp::Divide, src1, src2, dst) => {
            ainstrs.push(AInstr::Mov(operand(&src1), AOprnd::Reg(AReg::AX)));
            ainstrs.push(AInstr::Cdq);
            ainstrs.push(AInstr::Idiv(operand(&src2)));
            ainstrs.push(AInstr::Mov(AOprnd::Reg(AReg::AX), operand(&dst)));
        },
        TInstr::Binary(TBinaryOp::Remainder, src1, src2, dst) => {
            ainstrs.push(AInstr::Mov(operand(&src1), AOprnd::Reg(AReg::AX)));
            ainstrs.push(AInstr::Cdq);
            ainstrs.push(AInstr::Idiv(operand(&src2)));
            ainstrs.push(AInstr::Mov(AOprnd::Reg(AReg::DX), operand(&dst)));
        },
        TInstr::Binary(op, src1, src2, dst) => {
            ainstrs.push(AInstr::Mov(operand(&src1), operand(&dst)));
            ainstrs.push(AInstr::Binary(binary_op(&op), operand(&src2), operand(&dst)));
        }
    };
}

fn operand(val: &TVal) -> AOprnd {
    match val {
        TVal::Constant(val) => AOprnd::Imm(*val),
        TVal::Var(ident) => AOprnd::Pseudo(ident.to_string()),
    }
}

fn unary_op(op: &TUnaryOp) -> AUnaryOp {
    match op {
        TUnaryOp::Complement => AUnaryOp::Not,
        TUnaryOp::Negate => AUnaryOp::Neg,
    }
}

fn binary_op(op: &TBinaryOp) -> ABinaryOp {
    match op {
        TBinaryOp::Add => ABinaryOp::Add,
        TBinaryOp::Subtract => ABinaryOp::Sub,
        TBinaryOp::Multiply => ABinaryOp::Mult,
        _ => panic!("Invalid binary op for assembly binary op"),
    }
}