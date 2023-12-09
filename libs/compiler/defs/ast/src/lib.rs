mod defs;
mod exprs;
mod stmts;
mod util;

pub use defs::*;
pub use exprs::*;
pub use stmts::*;
pub use util::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstContext {
    pub modules: Vec<AstModule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstModule {
    pub name: String,
    pub dirs: Vec<AstDir>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDir {
    pub name: String,
    pub dirs: Vec<AstDir>,
    pub source: Vec<AstSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstSource {
    pub name: String,
    pub defs: Vec<AstDef>,
}
