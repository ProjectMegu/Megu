mod parsers;
mod tokens;

pub fn parse(code: &str) -> anyhow::Result<Vec<ast::AstDef>> {
    let tokens = tokens::lexer(code);
    debug!(&tokens);
    let parse = parsers::megu_parse(&tokens);

    match parse {
        Err(err) => {
            debug!(&err);
            anyhow::bail!("parse error: {:?}", err);
        }
        Ok(ast) => {
            debug!(&ast);
            Ok(ast)
        }
    }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($x:expr) => {
        dbg!($x)
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($x:expr) => {
        std::convert::identity($x)
    };
}
