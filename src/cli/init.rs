use allay::Project;
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("init")
        .about("Create a new Allay project")
        .arg(
            Arg::new("dir")
                .help("Directory to initialize")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("gitignore")
                .long("no-gitignore")
                .help("Prevents creation of `.gitignore` file")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force creation of files and directories even if the directory is not empty")
                .action(ArgAction::SetTrue),
        )
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let path: &PathBuf = matches.get_one("dir").unwrap();
    let force: bool = matches.get_flag("force");
    let with_gitignore: bool = matches.get_flag("gitignore");

    Project::new(
        path,
        force,
        allay::project::ProjectInitConfig { with_gitignore },
    )
    .expect("Failed do initialize project");

    ExitCode::SUCCESS
}
