use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstDef {
    Func(AstDefFunc),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDefFunc {
    pub name: String,
    pub args: Vec<AstDefFuncArg>,
    pub ret: Option<AstType>,
    pub inner: Vec<AstStmt>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDefFuncArg {
    pub name: String,
    pub arg_type: AstType,
}


