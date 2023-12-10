use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstStmt {
    Expr(AstExpr),
    DefV(AstDefValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDefValue {
    pub is_mut: bool,
    pub name: String,
    pub v_type: AstType,
    pub value: AstExpr,
}
