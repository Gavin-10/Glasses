
#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register,
}

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret
}

#[derive(Debug)]
pub enum AssemFunctionDef {
    Function(String, Vec<Instruction>),
}