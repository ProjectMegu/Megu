#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstExpr {
    CallFunc(CallFunc)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallFunc {
    pub name: Vec<String>,
    pub args: Vec<AstExpr>,
}