use hir::HirCtx;

pub fn pass_nspace(mut ctx: HirCtx) -> HirCtx {
    // self置き換え
    for module in &mut ctx.mods {
        for item in module.file_item.values_mut() {
            if item.line_nspace.name[0] == "self" {
                item.line_nspace.name[0] = module.name.clone();
            }

            for use_item in &mut item.use_ {
                if use_item.name[0] == "self" {
                    use_item.name[0] = module.name.clone();
                }
            }
        }
    }

    // Change NameSpace(itemごと)
    for module in &mut ctx.mods {
        let items = module.items.clone();
        for (nspace, item) in items {
            let data = module.file_item.get(&item.place);
            if let Some(data) = data {
                if !data.line_nspace.name.is_empty() {
                    // 既存のitemを削除
                    module.items.remove(&nspace);
                    let mut name = data.line_nspace.name.clone();
                    // アイテム名を追加
                    name.push(nspace[1].clone());
                    module.items.insert(name, item);
                }
            }
        }
    }

    ctx
}
