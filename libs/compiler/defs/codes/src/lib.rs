#[derive(Debug, Clone, PartialEq)]
pub struct CodeContext {
    pub modules: Vec<CodeModule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeModule {
    pub name: String,
    pub dirs: CodeDir,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CodeDir {
    pub name: String,
    pub dirs: Vec<CodeDir>,
    pub source: Vec<CodeSource>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CodeSource {
    pub name: String,
    pub code: String,
}
