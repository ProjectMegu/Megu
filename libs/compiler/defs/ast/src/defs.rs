use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstDef {
    Func(AstDefFunc),
    NSpace(AstBlockNamespace),
    LineNSpace(AstLineNamespace),
    Use(AstUse),
}

// DefFunc

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDefFunc {
    pub name: String,
    pub args: Vec<AstDefFuncArg>,
    pub ret: Option<AstType>,
    pub inner: Vec<AstStmt>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstDefFuncArg {
    pub name: String,
    pub arg_type: AstType,
}

// namespaces

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstNameSpaceTree {
    pub name: Vec<String>,
    pub relative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstLineNamespace {
    pub tree: AstNameSpaceTree,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstBlockNamespace {
    pub tree: AstNameSpaceTree,
    pub inner: Vec<AstDef>
}

// use

pub type AstUse = Vec<AstNameSpaceTree>;
