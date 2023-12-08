use std::{path::PathBuf, fs::File, io::Read};
use clap::Parser;
use parser::parse;

#[derive(Debug,Parser)]
struct Cmd {
   path: PathBuf
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
    let _ast = parse(&code);
    // let _ = dbg!(ast);
}
