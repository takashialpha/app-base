mod args;
pub use args::*;
use clap::Parser;

pub fn parse() -> CliArgs {
    CliArgs::parse()
}
