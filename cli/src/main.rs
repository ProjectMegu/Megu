mod build;
mod ws;

use build::parse_build;
use clap::{Parser, Subcommand};
use ws::parse_ws;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    cmds: Cmds,
}

#[derive(Debug, Subcommand)]
enum Cmds {
    #[command(subcommand)]
    WS(ws::WSCmds),

    Build {
        #[arg(short, long)]
        package: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmds {
        Cmds::WS(ws) => parse_ws(ws)?,
        Cmds::Build { package } => parse_build(package)?,
    }

    Ok(())
}
