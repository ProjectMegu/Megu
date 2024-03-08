use hir::{HirCtx, HirMod};

pub fn pass_nspace(mut ctx: HirCtx) -> HirCtx {
    // self置き換え
    for module in &mut ctx.mods {
        change_self(module);
    }

    // Change NameSpace(itemごと)
    for module in &mut ctx.mods {
        change_item_nspace(module);
    }

    ctx
}

fn change_self(module: &mut HirMod) {
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

fn change_item_nspace(module: &mut HirMod) {
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

// itemの宣言をUseに変換する
// ```megu
// fn Example() []
// ->
// use self.Example
// ```
// fn into_use(mut ctx: HirCtx) -> HirCtx {

// }

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use hir::{HirFileItem, HirMod, HirNameSpaceTree};

    #[test]
    fn test_change_self() {
        let mut test_file_item = HashMap::new();
        test_file_item.insert(
            vec!["__ROOT__".to_string(), "Main.meg".to_string()],
            HirFileItem {
                line_nspace: HirNameSpaceTree {
                    name: vec!["self".to_string(), "Test".to_string()],
                },
                use_: vec![HirNameSpaceTree {
                    name: vec!["self".to_string(), "Func".to_string()],
                }],
            },
        );

        let mut md_test = HirMod {
            name: "Test".to_string(),
            items: Default::default(),
            file_item: test_file_item,
        };

        let mut expect_file_item = HashMap::new();
        expect_file_item.insert(
            vec!["__ROOT__".to_string(), "Main.meg".to_string()],
            HirFileItem {
                line_nspace: HirNameSpaceTree {
                    name: vec!["Test".to_string(), "Test".to_string()],
                },
                use_: vec![HirNameSpaceTree {
                    name: vec!["Test".to_string(), "Func".to_string()],
                }],
            },
        );

        let expect_md = HirMod {
            name: "Test".to_string(),
            items: Default::default(),
            file_item: expect_file_item,
        };

        super::change_self(&mut md_test);
        assert_eq!(md_test, expect_md);
    }
}
