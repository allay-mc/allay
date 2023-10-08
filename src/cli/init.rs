use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::build::uuidgen;
use crate::environment::Environment;
use crate::template;
use crate::utils::empty_dir;

pub(crate) fn cmd() -> Command {
    Command::new("init")
        .about("Initialises an Allay project in an empty directory")
        .visible_alias("i")
        .arg(
            Arg::new("path")
                .help("Specify the directory which should be initialized. Uses the current directory by default")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force creation of files and directories even if the directory is not empty")
                .action(clap::ArgAction::SetTrue)
        )
}

pub(crate) fn run(matches: &ArgMatches, _env: &mut Environment) {
    let cwd: &PathBuf = &std::env::current_dir()
        .expect("cannot acquire current directory; consider explicitly specifying the path");
    let path: &PathBuf = matches.get_one("path").unwrap_or(cwd);
    let force: &bool = matches.get_one("force").unwrap();

    if !path.is_dir() {
        panic!("'{}' is not a directory", path.display());
    };

    if !force && !empty_dir(path).unwrap() {
        panic!(
            "'{}' needs to be empty in order to be initialized as an Allay project",
            path.display()
        );
    }

    template::create_template(path.as_path()).expect("failed to create template");
    uuidgen::save_uuids(&uuidgen::new()).expect("cannot save UUIDs");
}
