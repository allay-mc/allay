use crate::{paths, Health};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("health")
        .about("Show the status of the project")
        .arg(
            Arg::new("fix")
                .long("fix")
                .help("Fixes malformed project artifacts")
                .action(ArgAction::SetTrue),
        )
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let fix = matches.get_flag("fix");

    let root = paths::root();
    let health = Health { root, fix };
    let success = health.check_all();

    if success {
        log::info!("Everything seems functional");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
