mod defs;
mod util;
mod stmts;
mod exprs;

pub use defs::*;
pub use util::*;
pub use stmts::*;
pub use exprs::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstContext {
    pub modules: Vec<AstModule>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstModule {
    pub name: String,
    pub dirs: Vec<AstDir>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDir {
    pub name: String,
    pub dirs: Vec<AstDir>,
    pub source: Vec<AstSource>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstSource {
    pub name: String,
    pub defs: Vec<AstDef>
}