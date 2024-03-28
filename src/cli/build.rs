use super::prelude::*;
use allay::project::Project;
use clap::{ArgMatches, Command};
use std::process::ExitCode;
use std::time::Instant;

pub fn cmd() -> Command {
    Command::new("build")
        .visible_alias("b")
        .about("Build the add-ons")
        .arg_build_opts()
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let debug_mode: Option<bool> = matches
        .get_flag("build-debug")
        .then(|| true)
        .or(matches.get_flag("build-release").then(|| false));
    let now = Instant::now();
    let mut project = Project::current().unwrap();
    if let Some(debug_mode) = debug_mode {
        project.config.debug = debug_mode;
    }
    project.build().expect("failed to build");
    let took = now.elapsed().as_millis();
    log::info!("Built project in {}ms", took);
    ExitCode::SUCCESS
}
