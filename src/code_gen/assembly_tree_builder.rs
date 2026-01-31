
use crate::code_gen::assembly_ast::*;
use crate::parser::ast::*;

pub fn gen_code(ast: FunctionDef) -> AssemFunctionDef {
    let assembly_tree = function_decl(&ast);

    assembly_tree
}

fn function_decl(ast: &FunctionDef) -> AssemFunctionDef {
    match ast {
        FunctionDef::Function(identifier, body) => AssemFunctionDef::Function(identifier.to_string(), instructions(&body)),
    }
}

fn instructions(body: &Body) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    match body {
        Body::Return(expr) => {
            instructions.push(Instruction::Mov(expression(&expr), Operand::Register));
            instructions.push(Instruction::Ret);
        },
    };

    instructions
}

fn expression(expr: &Expr) -> Operand {
    match expr {
        Expr::Constant(val) => Operand::Imm(*val),
    }
}