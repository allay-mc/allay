use crate::project::{Project, ProjectInitConfig};
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
            Arg::new("init-git")
                .long("no-git")
                .help("Prevents creation of a git repository")
                .action(ArgAction::SetFalse)
                .hide(!cfg!(feature = "git")),
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
    #[cfg(feature = "git")]
    let init_git: bool = matches.get_flag("init-git");

    Project::new(
        path,
        force,
        ProjectInitConfig {
            with_gitignore,
            #[cfg(feature = "git")]
            init_git,
        },
    )
    .expect("Failed do initialize project");

    ExitCode::SUCCESS
}
