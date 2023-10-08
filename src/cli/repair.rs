use clap::{Arg, ArgMatches, Command};

use crate::{environment::Environment, health};

pub(crate) fn cmd() -> Command {
    Command::new("repair")
        .about("Repairs the project where necessary")
        .visible_alias("rep")
}

pub(crate) fn run(_matches: &ArgMatches, _env: &mut Environment) {
    health::repair().expect("cannot repair project");
}
