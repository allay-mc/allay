use clap::{Arg, ArgMatches, Command};

pub(crate) fn cmd() -> Command {
    Command::new("config").about("Manages configuration of the Allay project")
}

pub(crate) fn run(matches: &ArgMatches) {
    todo!();
}
