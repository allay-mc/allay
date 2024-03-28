use allay::Project;
use clap::{ArgMatches, Command};
use prettytable::format::consts::*;
use std::process::ExitCode;

mod refresh;

pub fn cmd() -> Command {
    Command::new("uuid")
        .about("Manages the project's UUIDs")
        .subcommands([refresh::cmd()]) // TODO: command for adding dependencies
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    match matches.subcommand() {
        Some(("refresh", m)) => refresh::run(m),
        _ => {
            let project = Project::current().unwrap();
            let mut uuids: prettytable::Table = project.uuids.into();
            uuids.set_format(*FORMAT_BOX_CHARS);
            uuids.print_tty(false).unwrap();
        }
    }
    ExitCode::SUCCESS
}
