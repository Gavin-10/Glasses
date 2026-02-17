
#[derive(Debug)]
#[derive(Clone)]
pub enum TUnaryOp {
    Complement,
    Negate,
    Not,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum TBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreatThan,
    GreatEqual,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum TVal {
    Constant(i32),
    Var(String),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum TInstr {
    Return(TVal),
    Unary(TUnaryOp, TVal, TVal),
    Binary(TBinaryOp, TVal, TVal, TVal),
    Copy(TVal, TVal),
    Jump(String),
    JumpIfZero(TVal, String),
    JumpIfNotZero(TVal, String),
    Label(String),
}

#[derive(Debug)]
pub enum TFuncDef {
    Function(String, Vec<TInstr>),
}