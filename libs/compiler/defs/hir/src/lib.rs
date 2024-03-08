pub mod internals;
pub use crate::internals::*;
pub mod file_items;
pub use crate::file_items::*;
pub mod items;
pub use crate::items::*;
pub mod exprs;
pub use crate::exprs::*;

use std::collections::HashMap;
use utils::SccMap;

#[derive(Debug, Clone, Default)]
pub struct HirCtx {
    pub mods: Vec<HirMod>,
    pub deps: SccMap<String>,
}

pub type NameSpace = Vec<String>;
pub type FilePlace = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub struct HirMod {
    pub name: String,
    pub items: HashMap<NameSpace, HirItem>,
    pub file_item: HashMap<FilePlace, HirFileItem>,
}
