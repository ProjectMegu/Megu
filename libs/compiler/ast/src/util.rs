#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    Value(Vec<String>),
    Unknown,
}