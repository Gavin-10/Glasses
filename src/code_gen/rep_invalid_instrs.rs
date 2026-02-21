
use crate::code_gen::a_ast::*;

pub fn rep_instrs(ast: &mut AssemFuncDef, stack_size: i32) {
    check_func(ast, stack_size);
}

fn check_func(ast: &mut AssemFuncDef, stack_size: i32) {
    match ast {
        AssemFuncDef::Function(_, instrs) => {
            *instrs = check_instructions(&instrs);
            instrs.insert(0, AInstr::AllocateStack(stack_size));
        },
    };
}

fn check_instructions(instrs: &Vec<AInstr>) -> Vec<AInstr> {
    let mut new_instrs: Vec<AInstr> = Vec::new();

    for instr in instrs.iter() {
        match instr {
            AInstr::Mov(AOprnd::Stack(src), AOprnd::Stack(dst)) => {
                new_instrs.push(AInstr::Mov(AOprnd::Stack(*src), AOprnd::Reg(AReg::R10)));
                new_instrs.push(AInstr::Mov(AOprnd::Reg(AReg::R10), AOprnd::Stack(*dst)));
            },
            AInstr::Idiv(AOprnd::Imm(val)) => {
                new_instrs.push(AInstr::Mov(AOprnd::Imm(*val), AOprnd::Reg(AReg::R10)));
                new_instrs.push(AInstr::Idiv(AOprnd::Reg(AReg::R10)));
            },
            AInstr::Binary(_, _, _) => check_binary(&instr, &mut new_instrs),
            AInstr::Cmp(_, _) => check_cmp(&instr, &mut new_instrs),
            _ => new_instrs.push(instr.clone()),
        };
    }

    new_instrs
}

fn check_binary(instr: &AInstr, new_instrs: &mut Vec<AInstr>) {
    match instr {
        AInstr::Binary(ABinaryOp::Add, AOprnd::Stack(src), AOprnd::Stack(dst)) => {
            new_instrs.push(AInstr::Mov(AOprnd::Stack(*src), AOprnd::Reg(AReg::R10)));
            new_instrs.push(AInstr::Binary(ABinaryOp::Add, AOprnd::Reg(AReg::R10), AOprnd::Stack(*dst)));
        },
        AInstr::Binary(ABinaryOp::Sub, AOprnd::Stack(src), AOprnd::Stack(dst)) => {
            new_instrs.push(AInstr::Mov(AOprnd::Stack(*src), AOprnd::Reg(AReg::R10)));
            new_instrs.push(AInstr::Binary(ABinaryOp::Sub, AOprnd::Reg(AReg::R10), AOprnd::Stack(*dst)));
        },
        AInstr::Binary(ABinaryOp::Mult, src_instr, AOprnd::Stack(dst)) => {
            new_instrs.push(AInstr::Mov(AOprnd::Stack(*dst), AOprnd::Reg(AReg::R11)));
            match src_instr {
                AOprnd::Stack(src) => new_instrs.push(AInstr::Binary(ABinaryOp::Mult, AOprnd::Stack(*src), AOprnd::Reg(AReg::R11))),
                AOprnd::Imm(val) => new_instrs.push(AInstr::Binary(ABinaryOp::Mult, AOprnd::Imm(*val), AOprnd::Reg(AReg::R11))),
                _ => panic!("Binary multiplication instruction fix fail"),
            }
            new_instrs.push(AInstr::Mov(AOprnd::Reg(AReg::R11), AOprnd::Stack(*dst)));
        },
        _ => new_instrs.push(instr.clone()),
    }
}

fn check_cmp(instr: &AInstr, new_instrs: &mut Vec<AInstr>) {
    match instr {
        AInstr::Cmp(AOprnd::Stack(src1), AOprnd::Stack(src2)) => {
            new_instrs.push(AInstr::Mov(AOprnd::Stack(*src1), AOprnd::Reg(AReg::R10)));
            new_instrs.push(AInstr::Cmp(AOprnd::Reg(AReg::R10), AOprnd::Stack(*src2)));
        },
        AInstr::Cmp(src, AOprnd::Imm(val)) => {
            new_instrs.push(AInstr::Mov(AOprnd::Imm(*val), AOprnd::Reg(AReg::R11)));
            new_instrs.push(AInstr::Cmp(src.clone(), AOprnd::Reg(AReg::R11)));
        },
        _ => new_instrs.push(instr.clone()),
    }
}