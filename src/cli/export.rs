use super::build;
use super::prelude::*;
use allay::paths;
use clap::{ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("export")
        .visible_alias("x")
        .about("Export the add-on to Minecraft")
        .arg_build_opts()
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    build::run(matches);

    log::info!("Exporting add-on...");
    match open::that(paths::root().join(paths::build())) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            log::error!("Could not export add-on: {}", e);
            ExitCode::FAILURE
        }
    }
}
