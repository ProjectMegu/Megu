use logos::Logos;

pub(crate) fn lexer(input: &str) -> Vec<MeguToken<'_>> {
    MeguToken::lexer(input).flatten().collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Logos)]
pub enum MeguToken<'a> {
    // keywords
    // fn
    #[token("fn")]
    DefFN,
    // nspace
    #[token("nspace")]
    DefNSpace,
    // use
    #[token("use")]
    DefUse,

    // Parens
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,

    // corons
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,

    // dot
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    
    // whites
    #[regex(r"[ \t]*", logos::skip)]
    WhiteSpace,

    // comments
    #[regex(r"//.*", logos::skip)]
    Comment,
    // block comments
    #[regex(r"/\*[^(\*/)]*\*/", logos::skip)]
    BlockComment,

    // newlines
    #[regex(r"\n+")]
    NewLine,

    // Regexs
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),
}