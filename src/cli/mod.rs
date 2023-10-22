use clap::{Arg, ArgMatches, Command};

use crate::environment::Environment;

#[cfg(feature = "command-add")]
pub(crate) mod add;
pub(crate) mod build;
#[cfg(feature = "command-doc")]
pub(crate) mod doc;
pub(crate) mod init;
pub(crate) mod repair;
#[cfg(feature = "config-schema")]
pub(crate) mod schema;
pub(crate) mod uuid;

pub(crate) fn cmd() -> Command {
    Command::new("Allay")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(if std::env::var("NO_COLOR").is_ok() {
            clap::crate_description!()
        } else {
            concat!(
                "\x1b[46m \x1b[47m \x1b[46m \x1b[47m \x1b[46m \x1b[0m",
                " ",
                clap::crate_description!()
            )
        })
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("Enables verbosity in 5 different levels by repeating this flags")
                .action(clap::ArgAction::Count)
                .default_value("3"),
        )
        .subcommands([
            #[cfg(feature = "command-add")]
            add::cmd(),
            build::cmd(),
            #[cfg(feature = "command-doc")]
            doc::cmd(),
            init::cmd(),
            repair::cmd(),
            uuid::cmd(),
            #[cfg(feature = "config-schema")]
            schema::cmd(),
        ])
}

pub(crate) fn run(matches: &ArgMatches, env: &mut Environment) {
    let verbose: &u8 = matches.get_one("verbose").unwrap();
    let mut log_builder = env_logger::Builder::from_default_env();
    log_builder
        // TODO: always log to file, log to stdout if active
        .filter_level(match verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            5 => log::LevelFilter::Trace,
            _ => panic!("invalid verbosity level {}", verbose),
        })
        .init();

    match matches.subcommand() {
        #[cfg(feature = "command-add")]
        Some(("add", m)) => add::run(m, env),
        Some(("build", m)) => build::run(m, env),
        #[cfg(feature = "command-doc")]
        Some(("doc", m)) => doc::run(m, env),
        Some(("init", m)) => init::run(m, env),
        Some(("repair", m)) => repair::run(m, env),
        Some(("uuid", m)) => uuid::run(m, env),
        #[cfg(feature = "config-schema")]
        Some(("schema", m)) => schema::run(m, env),
        _ => unreachable!(),
    }
}
