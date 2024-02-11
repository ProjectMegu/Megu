use ast::{AstContext, AstDir, AstModule, AstSource};
use codes::{CodeContext, CodeDir};
use utils::bind_result;

pub fn megu_compile(ctx: CodeContext) -> anyhow::Result<()> {
    dbg!(&ctx);

    // 仮置き
    let _ = convert_ast_ctx(ctx)?;

    Ok(())
}

pub fn convert_ast_ctx(ctx: CodeContext) -> anyhow::Result<AstContext> {
    let mods: Vec<anyhow::Result<AstModule>> = ctx
        .modules
        .into_iter()
        .map(|a| {
            Ok(AstModule {
                name: a.name,
                root_dir: convert_ast(a.dirs)?,
            })
        })
        .collect();
    let mods = bind_result(mods.into_iter())?;
    Ok(AstContext { modules: mods, deps: ctx.deps})
}

pub fn convert_ast(dir: CodeDir) -> anyhow::Result<AstDir> {
    let mut sources = Vec::new();

    for s in dir.source {
        sources.push(AstSource {
            name: s.name,
            defs: parser::parse(&s.code)?,
        })
    }

    let mut dirs = Vec::new();

    for d in dir.dirs {
        dirs.push(convert_ast(d)?);
    }

    Ok(AstDir {
        name: dir.name,
        dirs,
        source: sources,
    })
}
