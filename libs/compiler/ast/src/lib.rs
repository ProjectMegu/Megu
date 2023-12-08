mod defs;
mod util;

pub use defs::*;
pub use util::*;

pub struct AstContext {
    pub modules: Vec<AstModule>
}

pub struct AstModule {
    pub name: String,
    pub dirs: Vec<AstDir>
}

pub struct AstDir {
    pub name: String,
    pub dirs: Vec<AstDir>,
    pub source: Vec<AstSource>
}

pub struct AstSource {
    pub name: String,
    pub defs: Vec<AstDef>
}