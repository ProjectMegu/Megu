//! Converts AST to HIR
//! 先にASTそのままのHIRを作り､Use解決は後回し
//! Use解決の前にNspaceを明示する

use ast::{AstContext, AstDef, AstDir};
use hir::{HirCtx, HirMod};

mod defs;
mod file_item;

pub fn into_hir(ast: AstContext) -> HirCtx {
    let mut res = HirCtx::default();

    // TODO: add deps

    for m in ast.modules {
        let (source_def, file_def) = bring_source(m.root_dir);

        res.mods.push(HirMod {
            name: m.name.clone(),
            items: defs::into_defs(source_def, m.name.clone()),
            file_item: file_item::into_file_item(file_def),
        });
    }

    res
}

type SourceItem = (Vec<String>, AstDef);
type FileItem = (Vec<String>, AstDef);

fn bring_source(dir: AstDir) -> (Vec<SourceItem>, Vec<FileItem>) {
    let mut res_item = Vec::new();
    let mut res_file = Vec::new();

    for s in dir.source {
        for def in s.defs {
            match def {
                AstDef::Func(_) | AstDef::NSpace(_) => {
                    res_item.push((vec![dir.name.clone(), s.name.clone()], def))
                }
                AstDef::LineNSpace(_) | AstDef::Use(_) => {
                    res_file.push((vec![dir.name.clone(), s.name.clone()], def))
                }
                _ => todo!(),
            }
        }
    }

    for dir in dir.dirs {
        let (a, b) = bring_source(dir);
        res_item.extend(a);
        res_file.extend(b);
    }

    (res_item, res_file)
}
