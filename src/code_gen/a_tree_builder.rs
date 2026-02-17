
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
        TInstr::Unary(TUnaryOp::Not, src, dst) => {
            ainstrs.push(AInstr::Cmp(AOprnd::Imm(0), operand(&src)));
            ainstrs.push(AInstr::Mov(AOprnd::Imm(0), operand(&dst)));
            ainstrs.push(AInstr::SetCC(CondCode::E, operand(&dst)));
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
        TInstr::Binary(TBinaryOp::GreatThan, src1, src2, dst) => conditional(CondCode::G, &src1, &src2, &dst, ainstrs),
        TInstr::Binary(TBinaryOp::GreatEqual, src1, src2, dst) => conditional(CondCode::GE, &src1, &src2, &dst, ainstrs),
        TInstr::Binary(TBinaryOp::LessThan, src1, src2, dst) => conditional(CondCode::L, &src1, &src2, &dst, ainstrs),
        TInstr::Binary(TBinaryOp::LessEqual, src1, src2, dst) => conditional(CondCode::LE, &src1, &src2, &dst, ainstrs),
        TInstr::Binary(TBinaryOp::Equal, src1, src2, dst) => conditional(CondCode::E, &src1, &src2, &dst, ainstrs),
        TInstr::Binary(TBinaryOp::NotEqual, src1, src2, dst) => conditional(CondCode::NE, &src1, &src2, &dst, ainstrs),
        TInstr::Binary(op, src1, src2, dst) => {
            ainstrs.push(AInstr::Mov(operand(&src1), operand(&dst)));
            ainstrs.push(AInstr::Binary(binary_op(&op), operand(&src2), operand(&dst)));
        },
        TInstr::JumpIfZero(val, target) => {
            ainstrs.push(AInstr::Cmp(AOprnd::Imm(0), operand(&val)));
            ainstrs.push(AInstr::JmpCC(CondCode::E, target.clone()));
        },
        TInstr::JumpIfNotZero(val, target) => {
            ainstrs.push(AInstr::Cmp(AOprnd::Imm(0), operand(&val)));
            ainstrs.push(AInstr::JmpCC(CondCode::NE, target.clone()));
        },
        TInstr::Jump(target) => ainstrs.push(AInstr::Jmp(target.clone())),
        TInstr::Copy(src, dst) => ainstrs.push(AInstr::Mov(operand(&src), operand(&dst))),
        TInstr::Label(val) => ainstrs.push(AInstr::Label(val.clone()))
    };
}

fn conditional(code: CondCode, src1: &TVal, src2: &TVal, dst: &TVal, ainstrs: &mut Vec<AInstr>) {
    ainstrs.push(AInstr::Cmp(operand(src2), operand(src1)));
    ainstrs.push(AInstr::Mov(AOprnd::Imm(0), operand(dst)));
    ainstrs.push(AInstr::SetCC(code, operand(&dst)));
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
        _ => todo!(),
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