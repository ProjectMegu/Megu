use ast::{AstContext, AstDef, AstDir};
use hir::{HirCtx, HirMod};

mod defs;

pub fn into_hir(ast: AstContext) -> HirCtx {
    let mut res = HirCtx { mods: Vec::new() };

    // TODO: add deps

    for m in ast.modules {
        res.mods.push(HirMod {
            name: m.name.clone(),
            items: defs::into_defs(bring_source(m.dirs), m.name),
        })
    }

    res
}

fn bring_source(dir: AstDir) -> Vec<(Vec<String>, AstDef)> {
    let mut res = Vec::new();

    for s in dir.source {
        res.append(
            &mut s
                .defs
                .into_iter()
                .map(|d| (vec![dir.name.clone(), s.name.clone()], d))
                .collect(),
        )
    }

    for d in dir.dirs {
        let mut a = bring_source(d)
            .into_iter()
            .map(|(mut a, b)| {
                a.insert(0, dir.name.clone());
                (a, b)
            })
            .collect::<Vec<_>>();
        res.append(&mut a)
    }

    res
}
