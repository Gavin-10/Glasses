
#[derive(Debug)]
#[derive(Clone)]
pub enum TUnaryOp {
    Complement,
    Negate,
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
}

#[derive(Debug)]
pub enum TFuncDef {
    Function(String, Vec<TInstr>),
}