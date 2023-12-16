use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AstDef {
    Func(AstDefFunc),
    NSpace(AstBlockNamespace),
    LineNSpace(AstLineNamespace),
    Use(AstUse),
}

// Attribute

#[derive(Debug, Clone, PartialEq)]
pub struct AstAttribute {
    pub name: Vec<String>,
    pub value: Vec<AstExpr>,
}

// DefFunc

#[derive(Debug, Clone, PartialEq)]
pub struct AstDefFunc {
    pub attr: Option<AstAttribute>,
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
    pub attr: Option<AstAttribute>,
    pub tree: AstNameSpaceTree,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstBlockNamespace {
    pub attr: Option<AstAttribute>,
    pub tree: AstNameSpaceTree,
    pub inner: Vec<AstDef>,
}

// use

pub type AstUse = Vec<AstNameSpaceTree>;
