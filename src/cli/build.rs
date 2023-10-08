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

pub(crate) fn run(matches: &ArgMatches, env: &mut Environment) {
    if env.config.is_none() {
        panic!("you are not in an allay project; initialize one with `allay init`");
    }
    env.development = Some(
        !matches.get_one("release").unwrap_or(&false)
            || env.config.as_ref().unwrap().project.development,
    );
    build(env).expect("failed to build project; try executing `allay repair`");
    log::info!("successfully built project");
}
