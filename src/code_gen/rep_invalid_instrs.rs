
use crate::code_gen::a_ast::*;

pub fn rep_instrs(ast: &mut AssemFuncDef, stack_size: i32) {
    check_func(ast, stack_size);
}

fn check_func(ast: &mut AssemFuncDef, stack_size: i32) {
    match ast {
        AssemFuncDef::Function(_, instrs) => {
            check_instructions(instrs);
            instrs.insert(0, AInstr::AllocateStack(stack_size));
        },
    };
}

fn check_instructions(instrs: &mut Vec<AInstr>) {
    let mut to_change: Vec<(usize, i32, i32)> = Vec::new();

    for (index, instr) in instrs.iter().enumerate() {
        match check_instruction(instr, index) {
            Some(val) => to_change.push(val),
            None => (),
        };
    }

    for change in to_change.iter() {
        change_instr(*change, instrs);
    }
}

fn check_instruction(instr: &AInstr, index: usize) -> Option<(usize, i32, i32)> {
    match instr {
        AInstr::Mov(AOprnd::Stack(src), AOprnd::Stack(dst)) => {
            Some((index, *src, *dst))
        },
        _ => None,
    }
}

fn change_instr(change: (usize, i32, i32), instrs: &mut Vec<AInstr>) {
    instrs[change.0] = AInstr::Mov(AOprnd::Stack(change.1), AOprnd::Reg(AReg::R10));
    instrs.insert(change.0 + 1, AInstr::Mov(AOprnd::Reg(AReg::R10), AOprnd::Stack(change.2)));
}