
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

    for (index, instr) in instrs.iter().enumerate() {
        match instr {
            AInstr::Mov(AOprnd::Stack(src), AOprnd::Stack(dst)) => {
                new_instrs.push(AInstr::Mov(AOprnd::Stack(*src), AOprnd::Reg(AReg::R10)));
                new_instrs.push(AInstr::Mov(AOprnd::Reg(AReg::R10), AOprnd::Stack(*dst)));
            },
            AInstr::Idiv(AOprnd::Imm(val)) => {
                new_instrs.push(AInstr::Mov(AOprnd::Imm(*val), AOprnd::Reg(AReg::R10)));
                new_instrs.push(AInstr::Idiv(AOprnd::Reg(AReg::R10)));
            },
            AInstr::Binary(ABinaryOp::Add, AOprnd::Stack(src), AOprnd::Stack(dst)) => {
                new_instrs.push(AInstr::Mov(AOprnd::Stack(*src), AOprnd::Reg(AReg::R10)));
                new_instrs.push(AInstr::Binary(ABinaryOp::Add, AOprnd::Reg(AReg::R10), AOprnd::Stack(*dst)));
            },
            AInstr::Binary(ABinaryOp::Sub, AOprnd::Stack(src), AOprnd::Stack(dst)) => {
                new_instrs.push(AInstr::Mov(AOprnd::Stack(*src), AOprnd::Reg(AReg::R10)));
                new_instrs.push(AInstr::Binary(ABinaryOp::Sub, AOprnd::Reg(AReg::R10), AOprnd::Stack(*dst)));
            },
            AInstr::Binary(ABinaryOp::Mult, _, AOprnd::Stack(dst)) => {
                if let AInstr::Binary(_, AOprnd::Stack(src), _) = instrs[index] {
                    new_instrs.push(AInstr::Mov(AOprnd::Stack(*dst), AOprnd::Reg(AReg::R11)));
                    new_instrs.push(AInstr::Binary(ABinaryOp::Mult, AOprnd::Stack(src), AOprnd::Reg(AReg::R11)));
                    new_instrs.push(AInstr::Mov(AOprnd::Reg(AReg::R11), AOprnd::Stack(*dst)));
                }

                if let AInstr::Binary(_, AOprnd::Imm(val), _) = instrs[index] {
                    new_instrs.push(AInstr::Mov(AOprnd::Stack(*dst), AOprnd::Reg(AReg::R11)));
                    new_instrs.push(AInstr::Binary(ABinaryOp::Mult, AOprnd::Imm(val), AOprnd::Reg(AReg::R11)));
                    new_instrs.push(AInstr::Mov(AOprnd::Reg(AReg::R11), AOprnd::Stack(*dst)));
                }
            },
            
            _ => new_instrs.push(instr.clone()),
        };
    }

    new_instrs
}