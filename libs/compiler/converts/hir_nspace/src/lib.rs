use hir::HirCtx;

pub fn pass_nspace(mut ctx: HirCtx) -> HirCtx {
    // Change NameSpace
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

    // Change Use
    for module in &mut ctx.mods {
        for item in module.file_item.values_mut() {
            for use_item in &mut item.use_ {
                if use_item.relative {
                    if !item.line_nspace.name.is_empty() {
                        if item.line_nspace.relative {
                            for n in item.line_nspace.name.iter().rev() {
                                use_item.name.insert(0, n.clone());
                            }
                            use_item.name.insert(0, module.name.clone());
                        } else {
                            for n in item.line_nspace.name.iter().rev() {
                                use_item.name.insert(0, n.clone());
                            }
                        }
                    } else {
                        use_item.name.insert(0, module.name.clone());
                    }
                }
            }
        }
    }

    ctx
}
