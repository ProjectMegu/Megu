use std::collections::HashMap;

use utils::SccMap;

#[derive(Debug, Clone, Default)]
pub struct HirCtx {
    pub mods: Vec<HirMod>,
    pub deps: SccMap<String>,
}

type NameSpace = Vec<String>;
type FilePlace = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub struct HirMod {
    pub name: String,
    pub items: HashMap<NameSpace, HirItem>,
    pub file_item: HashMap<FilePlace, HirFileItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirFileItem {
    pub line_nspace: HirNameSpaceTree,
    pub use_: Vec<HirNameSpaceTree>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirNameSpaceTree {
    pub name: Vec<String>,
    pub relative: bool,
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
    pub name: String,
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
