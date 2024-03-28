use clap::{ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("manual").about("Open the manual")
}

pub fn run(_matches: &ArgMatches) -> ExitCode {
    match open::that("https://allay-mc.github.io/allay") {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            log::error!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
