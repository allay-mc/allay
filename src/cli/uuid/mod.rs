use clap::{ArgMatches, Command};

use crate::environment::Environment;

pub(crate) mod refresh;

pub(crate) fn cmd() -> Command {
    Command::new("uuid")
        .about("Manages the project's UUIDs")
        .subcommands([refresh::cmd()])
}

pub(crate) fn run(matches: &ArgMatches, env: &mut Environment) {
    match matches.subcommand() {
        Some(("refresh", m)) => refresh::run(m),
        _ => {
            let uuids = &mut env.uuids;
            // TODO: styled
            uuids
                .as_mut()
                .unwrap()
                .set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
            uuids.as_ref().unwrap().print_tty(false).unwrap();
        }
    };
}
