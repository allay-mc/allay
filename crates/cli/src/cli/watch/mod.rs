#![cfg(feature = "watch")]

use super::ext::CommandExt;
use crate::utils;
use clap::{Arg, ArgMatches, Command};
use std::thread::sleep;
use std::{path::PathBuf, process, time::Duration};
mod poll;

pub(crate) fn cmd() -> Command {
    Command::new("watch")
        .about("Watch for changes and rebuild project")
        .arg(
            Arg::new("interval")
                .short('t')
                .long("interval")
                .help("The interval to use for watching for changes in seconds")
                .value_names(["SECONDS"])
                .value_parser(clap::value_parser!(f64))
                .default_value("1"),
        )
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

    let interval = Duration::from_secs_f64(*matches.get_one("interval").unwrap());
    let build_context = utils::build::build_context_from_args_and_config(matches, &project.config);

    let extra_watch = project.config.build.extra_watch.clone();
    let ignore_watch = project.config.build.ignore_watch.clone();

    let source_path = allay::paths::project::source(&project.root);
    let config_path = allay::paths::project::config(&project.root);

    let mut watcher = poll::Watcher::default();
    watcher
        .watch(&source_path)
        .watch(&config_path);
    for path in &extra_watch {
        watcher.watch(path);
    }
    for path in &ignore_watch {
        watcher.ignore(path);
    }

    loop {
        if !watcher.changes().is_empty() && project.build(&build_context).is_err() {
            log::error!("Failed to rebuild");
        }
        sleep(interval);
    }
}
