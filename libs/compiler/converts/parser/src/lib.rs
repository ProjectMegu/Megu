mod parsers;
mod tokens;

pub fn parse(code: &str) -> anyhow::Result<Vec<ast::AstDef>> {
    let tokens = tokens::lexer(code);
    let parse = parsers::megu_parse(&tokens);

    match parse {
        Err(err) => {
            anyhow::bail!("parse error: {:?}", err);
        }
        Ok(ast) => {
            Ok(ast)
        }
    }
}