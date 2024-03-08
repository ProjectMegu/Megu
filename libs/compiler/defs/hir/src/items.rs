use crate::*;
#[derive(Debug, Clone, PartialEq)]
pub struct HirItem {
    pub place: Vec<String>, // "__ROOT__/Main.meg"
    // pub attrs : Vec<HirAttr>,
    pub item_type: HirItemType,
    pub item_name: String,
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
