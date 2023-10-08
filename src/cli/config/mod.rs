use clap::{ArgMatches, Command};

use crate::environment::Environment;

pub(crate) fn cmd() -> Command {
    Command::new("config").about("Manages configuration of the Allay project")
}

pub(crate) fn run(matches: &ArgMatches, env: &mut Environment) {
    todo!();
}
