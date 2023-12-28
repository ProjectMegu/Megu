use std::collections::HashMap;

use ast::{AstDef, AstExpr, AstLitValues, AstStmt};
use hir::{HirCallFunc, HirExpr, HirItem, HirItemType, HirStmt};

pub(crate) fn into_defs(
    defs: Vec<(Vec<String>, AstDef)>,
    mod_name: String,
) -> HashMap<Vec<String>, HirItem> {
    let mut res = HashMap::new();

    for (place, def) in defs {
        match def {
            AstDef::Func(func) => {
                res.insert(
                    vec![mod_name.clone(), func.name /* 仮 */],
                    HirItem {
                        place,
                        item_type: HirItemType::Fn(hir::HirFn {
                            body: func.inner.into_iter().map(into_stmt).collect(),
                        }),
                    },
                );
            }
            _ => todo!(),
        }
    }

    res
}

pub(crate) fn into_stmt(stmt: AstStmt) -> HirStmt {
    match stmt {
        AstStmt::Expr(expr) => HirStmt::Expr(into_expr(expr)),
        _ => todo!(),
    }
}

pub(crate) fn into_expr(expr: AstExpr) -> HirExpr {
    match expr {
        AstExpr::CallFunc(func) => HirExpr::CallFunc(HirCallFunc {
            name: func.name,
            args: func.args.into_iter().map(into_expr).collect(),
        }),
        AstExpr::Lit(lit) => match lit {
            AstLitValues::Number(n) => HirExpr::LitInt(n),
            AstLitValues::String(s) => HirExpr::LitStr(s),
            _ => todo!(),
        },
    }
}
