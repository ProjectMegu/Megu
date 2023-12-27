use std::{
    fs::{self, File},
    io::Write,
};

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum WSCmds {
    Init,
    New {
        name: String,
        #[arg(short, long)]
        bin: bool,
        #[arg(short, long)]
        lib: bool,
    },
}

const WS_INIT_TOML: &str = r#"[context]
modules = [

]
"#;

const WS_NEW_BIN_TEMP: &str = r#"fn Main() [

]
"#;

const WS_NEW_LIB_TEMP: &str = r#"fn Add(a: Int,b: Int) [

]
"#;

pub fn parse_ws(ws: WSCmds) -> anyhow::Result<()> {
    match ws {
        WSCmds::Init => {
            println!("Workspace Init...");
            let mut f = File::create("Megu.toml")?;
            f.write_all(WS_INIT_TOML.as_bytes())?;
        }
        WSCmds::New { name, bin, lib } => {
            println!("Workspace Module New...: {}", name);
            let type_ = if bin {
                "bin"
            } else if lib {
                "lib"
            } else {
                "bin"
            };

            let toml = format!(
                r#"[module]
name = "{name}"
type = "{type_}"

# [[dep]]
# name = "DEPModule"
# path = {{ type = "local", path = "../DEPModule" }}
"#
            );

            // module root dir
            fs::create_dir(&name)?;
            // module setting file
            let mut f = File::create(format!("{}/Megu.toml", name))?;
            f.write_all(toml.as_bytes())?;
            // module main file
            if bin || !lib {
                let mut f = File::create(format!("{}/main.meg", name))?;
                f.write_all(WS_NEW_BIN_TEMP.as_bytes())?;
            } else {
                let mut f = File::create(format!("{}/mod.meg", name))?;
                f.write_all(WS_NEW_LIB_TEMP.as_bytes())?;
            }
        }
    }

    Ok(())
}
