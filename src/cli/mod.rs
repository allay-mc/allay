use clap::{Arg, ArgMatches, Command};

pub(crate) mod add;
pub(crate) mod build;
pub(crate) mod config;
pub(crate) mod init;
pub(crate) mod repair;
pub(crate) mod uuid;

macro_rules! ansi_allay {
    () => {
        "\x1b[46m \x1b[47m \x1b[46m \x1b[47m \x1b[46m \x1b[0m"
    };
}

pub(crate) fn cmd() -> Command {
    Command::new("Allay")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(concat!(ansi_allay!(), " ", clap::crate_description!()))
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("Enables verbosity in 5 different levels by repeating this flags")
                .action(clap::ArgAction::Count)
                .default_value("3"),
        )
        .subcommands([
            add::cmd(),
            build::cmd(),
            config::cmd(),
            init::cmd(),
            repair::cmd(),
            uuid::cmd(),
        ])
}

pub(crate) fn run(matches: &ArgMatches) {
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
        Some(("add", m)) => add::run(m),
        Some(("build", m)) => build::run(m),
        Some(("config", m)) => config::run(m),
        Some(("init", m)) => init::run(m),
        Some(("repair", m)) => repair::run(m),
        Some(("uuid", m)) => uuid::run(m),
        _ => unreachable!(),
    }
}
