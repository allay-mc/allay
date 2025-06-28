// TODO: init.rhai
// TODO: create num file before first build (on first build Allay fails because there is none)

use std::process::{self, ExitCode};
use logging::setup_logging;

mod cli;
mod init;
mod resources;
mod templates;
mod utils;
mod logging;

fn main() -> process::ExitCode {
    init::initialize().expect("Failed to initialize");
    
    let matches = cli::cmd().get_matches();
    let verbosity = if matches.get_flag("quiet") {
        0
    } else {
        matches.get_count("verbose")
    };
    if let Err(error) = setup_logging(verbosity) {
        eprintln!("{}", error);
        return ExitCode::FAILURE;
    }

    cli::run(&matches)
}


