
#[derive(Debug)]
pub enum AReg {
    AX,
    R10,
}

#[derive(Debug)]
pub enum AOprnd {
    Imm(i32),
    Reg(AReg),
    Pseudo(String),
    Stack(i32),
}

#[derive(Debug)]
pub enum AUnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum AInstr {
    Mov(AOprnd, AOprnd),
    Unary(AUnaryOp, AOprnd),
    AllocateStack(i32),
    Ret
}

#[derive(Debug)]
pub enum AssemFuncDef {
    Function(String, Vec<AInstr>),
}