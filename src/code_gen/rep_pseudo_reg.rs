
use std::collections::HashMap;

use crate::code_gen::a_ast::*;

pub fn rep_pseudo_regs(ast: &mut AssemFuncDef) -> i32 {
    let mut var_addresses: HashMap<String, i32> = HashMap::new();
    let mut depth = 0;

    check_func(ast, &mut var_addresses, &mut depth);

    depth.abs()
}

fn check_func(ast: &mut AssemFuncDef, var_addresses: &mut HashMap<String, i32>, depth: &mut i32) {
    match ast {
        AssemFuncDef::Function(_, instrs) => check_instructions(instrs, var_addresses, depth),
    };
}

fn check_instructions(instrs: &mut Vec<AInstr>, var_addresses: &mut HashMap<String, i32>, depth: &mut i32) {
    for instr in instrs.iter_mut() {
        check_instruction(instr, var_addresses, depth);
    }
}

fn check_instruction(instr: &mut AInstr, var_addresses: &mut HashMap<String, i32>, depth: &mut i32) {
    match instr {
        AInstr::Mov(src, dst) => {
            check_operand(src, var_addresses, depth);
            check_operand(dst, var_addresses, depth);
        },
        AInstr::Unary(_, op) => check_operand(op, var_addresses, depth),
        AInstr::Binary(_, op1, op2) => {
            check_operand(op1, var_addresses, depth);
            check_operand(op2, var_addresses, depth);
        },
        AInstr::Idiv(op) => check_operand(op, var_addresses, depth),
        _ => (),
    };
}

fn check_operand(op: &mut AOprnd, var_addresses: &mut HashMap<String, i32>, depth: &mut i32) {
    match op {
        AOprnd::Pseudo(val) => {
            let val_depth = match var_addresses.get(val) {
                Some(res) => res,
                None => {
                    *depth -= 4;
                    var_addresses.insert(val.to_string(), *depth);

                    depth
                }
            };

            *op = AOprnd::Stack(*val_depth);
        },
        _ => (),
    };
}