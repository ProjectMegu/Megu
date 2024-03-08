use crate::*;
#[derive(Debug, Clone, PartialEq)]
pub struct HirFileItem {
    pub line_nspace: HirNameSpaceTree,
    pub use_: Vec<HirNameSpaceTree>,
    pub refers: HirRefers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirNameSpaceTree {
    pub name: Vec<String>,
}
