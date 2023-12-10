use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstStmt {
    Expr(AstExpr),
}
