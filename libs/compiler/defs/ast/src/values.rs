#[derive(Debug, Clone, PartialEq)]
pub enum AstLitValues {
    Number(AstNumber),
    String(AstString),
}

pub type AstNumber = f64;

pub type AstString = String;
