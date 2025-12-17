use crate::config::ConfigError;

mod app;
mod cli;
mod config;

fn main() {
    if let Err(e) = init() {
        eprint!("{}", e);
        std::process::exit(1);
    }
}

fn init() -> Result<(), ConfigError> {
    let cli_args = cli::parse();
    let settings = config::load(cli_args.init.config)?;

    app::run(settings, cli_args.runtime);

    Ok(())
}
