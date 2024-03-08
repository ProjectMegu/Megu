use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    CallFunc(AstCallFunc),
    Lit(AstLitValues),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstCallFunc {
    pub name: String,
    pub args: Vec<AstExpr>,
}
