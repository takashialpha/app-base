use crate::{cli::RuntimeArgs, config::Settings};

pub fn run(settings: Settings, args: RuntimeArgs) {
    println!("{:?}", settings);
    println!("{:?}", args);
}
