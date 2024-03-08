use core::panic;
use std::collections::HashMap;

use ast::AstDef;
use hir::{HirFileItem, HirNameSpaceTree};

pub(crate) fn into_file_item(
    items: Vec<(Vec<String>, AstDef)>,
) -> HashMap<Vec<String>, HirFileItem> {
    let mut file_map = HashMap::new();

    for (place, def) in items {
        match def {
            AstDef::LineNSpace(nspace) => {
                if file_map.get(&place).is_some() {
                    let data: &mut HirFileItem = file_map.get_mut(&place).unwrap();

                    if !data.line_nspace.name.is_empty() {
                        // TODO: error handling
                        panic!("line namespace is only one at one file.")
                    }

                    data.line_nspace = HirNameSpaceTree {
                        name: nspace.tree.name,
                    };
                } else {
                    file_map.insert(
                        place,
                        HirFileItem {
                            line_nspace: HirNameSpaceTree {
                                name: nspace.tree.name,
                            },
                            use_: Vec::new(),
                        },
                    );
                }
            }
            AstDef::Use(use_) => {
                if file_map.get(&place).is_some() {
                    let data: &mut HirFileItem = file_map.get_mut(&place).unwrap();
                    data.use_.extend(
                        use_.into_iter()
                            .map(|ns| HirNameSpaceTree { name: ns.name }),
                    );
                } else {
                    file_map.insert(
                        place,
                        HirFileItem {
                            line_nspace: HirNameSpaceTree { name: Vec::new() },
                            use_: use_
                                .into_iter()
                                .map(|ns| HirNameSpaceTree { name: ns.name })
                                .collect(),
                        },
                    );
                }
            }
            _ => todo!(),
        }
    }

    file_map
}
