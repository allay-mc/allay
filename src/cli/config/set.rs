use crate::environment::Environment;
use clap::{Arg, ArgMatches, Command};

pub(crate) fn cmd() -> Command {
    Command::new("set").about("Overrides a value in the configuration file")
}

pub(crate) fn run(matches: &ArgMatches, env: &mut Environment) {
    todo!();
}
