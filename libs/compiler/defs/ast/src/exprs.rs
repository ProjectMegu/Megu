use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    CallFunc(CallFunc),
    Lit(AstLitValues),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallFunc {
    pub name: Vec<String>,
    pub args: Vec<AstExpr>,
}
