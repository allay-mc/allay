#![cfg(feature = "manual")]

use clap::{ArgMatches, Command};
use std::process;

pub(crate) fn cmd() -> Command {
    Command::new("docs")
        .visible_alias("manual")
        .about("Open documentation for the installed version of Allay")
}

pub(crate) fn run(_matches: &ArgMatches) -> process::ExitCode {
    match open::that(allay::paths::global::manual().join("index.html")) {
        Ok(()) => process::ExitCode::SUCCESS,
        Err(error) => {
            log::error!("Error while opening manual: {}", error);
            process::ExitCode::FAILURE
        }
    }
}
