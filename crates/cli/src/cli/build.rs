use crate::utils;

use super::ext::CommandExt;
use clap::{ArgMatches, Command};
use std::{path::PathBuf, process};

pub(crate) fn cmd() -> Command {
    Command::new("build")
        .visible_alias("b")
        .about("Build the Allay project")
        .arg_build_opts()
}

pub(crate) fn run(matches: &ArgMatches) -> process::ExitCode {
    let maybe_project = match matches.get_one::<&PathBuf>("project-dir") {
        Some(dir) => allay::Project::load_from_dir(dir),
        None => allay::Project::load_from_within(),
    };
    let mut project = match maybe_project {
        Ok(project) => project,
        Err(error) => {
            log::error!("{}", error);
            log::info!("Try `allay health` to automatically find and fix issues");
            return process::ExitCode::FAILURE;
        }
    };

    let build_context = utils::build::build_context_from_args_and_config(matches, &project.config);

    match utils::build::build_project(&mut project, &build_context) {
        Ok(_) => process::ExitCode::SUCCESS,
        Err(_) => process::ExitCode::FAILURE,
    }
}
