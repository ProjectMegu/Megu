mod defs;
mod exprs;
mod stmts;
mod util;
mod values;

pub use defs::*;
pub use exprs::*;
pub use stmts::*;
pub use util::*;
pub use values::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AstContext {
    pub modules: Vec<AstModule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstModule {
    pub name: String,
    pub root_dir: AstDir,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDir {
    pub name: String,
    pub dirs: Vec<AstDir>,
    pub source: Vec<AstSource>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstSource {
    pub name: String,
    pub defs: Vec<AstDef>,
}
