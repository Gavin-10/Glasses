
#[derive(Debug)]
#[derive(Clone)]
pub enum AReg {
    AX,
    R10,
    DX,
    R11,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum AOprnd {
    Imm(i32),
    Reg(AReg),
    Pseudo(String),
    Stack(i32),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum CondCode {
    E,
    NE,
    G,
    GE,
    L,
    LE,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum AUnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ABinaryOp {
    Add,
    Sub,
    Mult,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum AInstr {
    Mov(AOprnd, AOprnd),
    Unary(AUnaryOp, AOprnd),
    Binary(ABinaryOp, AOprnd, AOprnd),
    Cmp(AOprnd, AOprnd),
    Idiv(AOprnd),
    Cdq,
    Jmp(String),
    JmpCC(CondCode, String),
    SetCC(CondCode, AOprnd),
    Label(String),
    AllocateStack(i32),
    Ret
}

#[derive(Debug)]
pub enum AssemFuncDef {
    Function(String, Vec<AInstr>),
}