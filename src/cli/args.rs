use std::path::PathBuf;

use clap::Parser;

// Network probe CLI arguments

#[derive(Debug, Parser)]
#[command(name = "net-probe")]
#[command(version)]
#[command(about = "A minimal network probing tool", long_about = None)]
pub struct CliArgs {
    #[command(flatten)]
    pub init: InitArgs,

    #[command(flatten)]
    pub runtime: RuntimeArgs,
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    // used in init
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct RuntimeArgs {
    // used in runtime
    #[arg(short, long)]
    pub verbose: bool,
}
