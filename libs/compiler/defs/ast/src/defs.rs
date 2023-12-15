use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AstDef {
    Func(AstDefFunc),
    NSpace(AstBlockNamespace),
    LineNSpace(AstLineNamespace),
    Use(AstUse),
}

// DefFunc

#[derive(Debug, Clone, PartialEq)]
pub struct AstDefFunc {
    pub name: String,
    pub args: Vec<AstDefFuncArg>,
    pub ret: Option<AstType>,
    pub inner: Vec<AstStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstDefFuncArg {
    pub name: String,
    pub arg_type: AstType,
}

// namespaces

#[derive(Debug, Clone, PartialEq)]
pub struct AstNameSpaceTree {
    pub name: Vec<String>,
    pub relative: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLineNamespace {
    pub tree: AstNameSpaceTree,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstBlockNamespace {
    pub tree: AstNameSpaceTree,
    pub inner: Vec<AstDef>,
}

// use

pub type AstUse = Vec<AstNameSpaceTree>;
