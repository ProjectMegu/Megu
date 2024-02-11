use ast::{AstContext, AstDir, AstModule, AstSource};
use clap::Parser;
use parser::parse;
use std::{fs::File, io::Read, path::PathBuf};
use to_hir::into_hir;

#[derive(Debug, Parser)]
struct Cmd {
    path: PathBuf,
}

fn main() {
    let cmd = Cmd::parse();
    let code = {
        let mut data = String::new();
        File::open(&cmd.path)
            .unwrap_or_else(|_| panic!("can't load file: {}", cmd.path.display()))
            .read_to_string(&mut data)
            .unwrap_or_else(|_| panic!("can't load file: {}", cmd.path.display()));
        data
    };
    let _ast = dbg!(parse(&code)).unwrap();
    let _hir = dbg!(into_hir(AstContext {
        modules: vec![AstModule {
            name: "main".to_string(),
            root_dir: AstDir {
                name: "__ROOT__".to_string(),
                dirs: vec![],
                source: vec![AstSource {
                    name: "main.meg".to_string(),
                    defs: _ast,
                }],
            }
        }]
    }));
}
