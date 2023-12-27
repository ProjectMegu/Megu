mod parsers;
mod tokens;

pub fn parse(code: &str) -> anyhow::Result<Vec<ast::AstDef>> {
    let tokens = tokens::lexer(code);
    let parse = parsers::megu_parse(&tokens);

    match parse {
        Err(err) => {
            anyhow::bail!("parse error in input code: {:?}\nCode: {}", err, code);
        }
        Ok(ast) => Ok(ast),
    }
}
