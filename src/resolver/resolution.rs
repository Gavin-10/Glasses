
use crate::parser::ast::*;
use crate::resolver::var_resolver::*;

pub fn resolve(ast: &mut FuncDef) {
    resolve_vars(ast);
}