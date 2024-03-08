#[derive(Debug, Clone, PartialEq)]
pub enum HirStmt {
    Expr(HirExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirExpr {
    CallFunc(HirCallFunc),
    LitStr(String),
    LitInt(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirCallFunc {
    pub name: String,
    pub args: Vec<HirExpr>,
}
