
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