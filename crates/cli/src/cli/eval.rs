use std::{path::PathBuf, process};

use clap::{Arg, ArgMatches, Command};

pub(crate) fn cmd() -> Command {
    Command::new("eval")
        .about("Evaluate a filter from the command-line")
        .arg(Arg::new("expression").help("The expression to evaluate").required(true))
}

pub(crate) fn run(matches: &ArgMatches) -> process::ExitCode {
    let expr: &String = matches.get_one("expression").unwrap();
    let maybe_project = match matches.get_one::<&PathBuf>("project-dir") {
        Some(dir) => allay::Project::load_from_dir(dir),
        None => allay::Project::load_from_within(),
    };
    let project = match maybe_project {
        Ok(project) => project,
        Err(error) => {
            log::error!("{}", error);
            log::info!("Try `allay health` to automatically find and fix issues");
            return process::ExitCode::FAILURE;
        }
    };

    let engine = allay::plugin::engine(&project.config.env);
    match engine.eval_expression::<bool>(expr) {
        Ok(true) => {
            log::info!("Plugin evaluated TRUE");
        },
        Ok(false) => {
            log::info!("Plugin evaluated FALSE");
        }
        Err(error) => {
            log::error!("Failed to evaluate filter: {}", error);
            return process::ExitCode::FAILURE;
        }
    };

    process::ExitCode::SUCCESS
}
