use clap::{Arg, ArgMatches, Command};

use crate::health;

pub(crate) fn cmd() -> Command {
    Command::new("repair")
        .about("Repairs the project where necessary")
        .visible_alias("rep")
}

pub(crate) fn run(_matches: &ArgMatches) {
    health::repair().expect("cannot repair project");
}
