use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct HirCtx {
    pub mods: Vec<HirMod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirMod {
    pub name: String,
    pub items: HashMap<Vec<String>, HirItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirItem {
    pub place: Vec<String>, // "__ROOT__/Main.meg"
    // pub attrs : Vec<HirAttr>,
    pub item_type: HirItemType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HirItemType {
    Fn(HirFn),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirFn {
    // pub params: Vec<HirFnParam>,
    pub body: Vec<HirStmt>,
}

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
    pub name: Vec<String>,
    pub args: Vec<HirExpr>,
}
