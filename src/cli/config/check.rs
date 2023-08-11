use clap::{Arg, ArgMatches, Command};

pub(crate) fn cmd() -> Command {
    Command::new("check").about("Checks whether the configuration file is valid")
}

pub(crate) fn cli(matches: &ArgMatches) {
    todo!();
}
