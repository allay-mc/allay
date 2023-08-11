use clap::{ArgMatches, Command};

use crate::environment::Environment;

pub(crate) mod refresh;

pub(crate) fn cmd() -> Command {
    Command::new("uuid")
        .about("Manages the project's UUIDs")
        .subcommands([refresh::cmd()])
}

pub(crate) fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("refresh", m)) => refresh::run(m),
        _ => {
            let mut uuids = Environment::new().uuids;
            // TODO: styled
            uuids.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
            uuids.print_tty(false).unwrap();
        }
    };
}
