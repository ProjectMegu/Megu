use crate::util::AstType;

#[derive(Debug, Clone, PartialEq)]
pub enum AstDef {
    Func(AstDefFunc)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDefFunc {
    pub name: String,
    pub args: Vec<AstDefFuncArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDefFuncArg {
    pub name: String,
    pub arg_type: AstType,
}


