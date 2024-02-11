use hir::HirCtx;

pub fn pass_nspace(mut ctx: HirCtx) -> HirCtx {
    for module in &mut ctx.mods {
        let items = module.items.clone();
        for (nspace, item) in items {
            let data = module.file_item.get(&item.place);
            if let Some(data) = data {
                if !data.line_nspace.name.is_empty() {
                    module.items.remove(&nspace);
                    if data.line_nspace.relative {
                        let mut name = vec![module.name.clone()];
                        name.extend(data.line_nspace.name.clone());
                        name.push(nspace[1].clone());
                        module.items.insert(name, item);
                    } else {
                        let mut name = data.line_nspace.name.clone();
                        name.push(nspace[1].clone());
                        module.items.insert(name, item);
                    }
                }
            }
        }
    }

    ctx
}
