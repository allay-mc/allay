use clap::{Arg, ArgMatches, Command};

use crate::build::build;
use crate::environment::Environment;

pub(crate) fn cmd() -> Command {
    Command::new("build")
        .about("Builds the add-on")
        .visible_alias("b")
        .arg(
            Arg::new("release")
                .long("release")
                .help("Builds the add-on in release mode")
                .value_parser(clap::value_parser!(bool)),
        )
}

pub(crate) fn run(matches: &ArgMatches) {
    let mut env = Environment::new();
    env.development =
        !matches.get_one("release").unwrap_or(&false) || env.config.project.development;
    build(&mut env);
}
