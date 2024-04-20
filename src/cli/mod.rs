mod build;
#[cfg(feature = "shell-completions")]
mod completions;
mod explain;
#[cfg(feature = "export")]
mod export;
mod health;
mod init;
mod logs;
#[cfg(feature = "manual")]
mod manual;
mod prelude;
#[cfg(feature = "config-schema")]
mod schema;
#[cfg(feature = "share")]
mod share;
mod sync;
mod uuid;
#[cfg(feature = "watch")]
mod watch;

use crate::paths;
use clap::{Arg, ArgAction, ArgMatches, Command};
use log::{Level, LevelFilter};
use simplelog::{
    Color, ColorChoice, CombinedLogger, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::process::ExitCode;
use std::{fs::File, panic};
use textwrap_macros::dedent;

pub fn cmd() -> Command {
    Command::new("allay")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(if std::env::var("NO_COLOR").is_ok() {
            clap::crate_description!()
        } else {
            concat!(
                "\x1b[46m \x1b[47m \x1b[46m \x1b[47m \x1b[46m \x1b[0m", // Allay ANSI art
                " ",
                clap::crate_description!()
            )
        })
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("Enables verbosity in 5 different levels by repeating this flags")
                .long_help(dedent!(
                    "
                    Enables verbosity in one of 5 different levels

                    -v      errors
                    -vv     + warnings
                    -vvv    + info
                    -vvvv   + debug
                    -vvvvv  + trace

                    --quiet will disable logs in the console but they will remain in log files
                "
                ))
                .action(clap::ArgAction::Count)
                .default_value("3"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Disables outputs to the console")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("license")
                .long("license")
                .help("Displays the license of Allay")
                .action(ArgAction::SetTrue),
        )
        .subcommands([
            build::cmd(),
            #[cfg(feature = "shell-completions")]
            completions::cmd(),
            explain::cmd(),
            #[cfg(feature = "export")]
            export::cmd(),
            health::cmd(),
            init::cmd(),
            logs::cmd(),
            #[cfg(feature = "manual")]
            manual::cmd(),
            #[cfg(feature = "config-schema")]
            schema::cmd(),
            #[cfg(feature = "share")]
            share::cmd(),
            sync::cmd(),
            uuid::cmd(),
            #[cfg(feature = "watch")]
            watch::cmd(),
        ])
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    if matches.get_flag("license") {
        println!("{}", include_str!("../../LICENSE.txt"));
        return ExitCode::SUCCESS;
    };

    if let Err(e) = CombinedLogger::init({
        let tm = time::OffsetDateTime::now_local()
            .expect("failed to configure time while initializing logger")
            .format(&time::format_description::well_known::Rfc3339)
            .expect("failed to format time while initializing logger");
        let log_file_name = format!("allay-log-{}.log", tm);
        let f = File::create(paths::logs().join(log_file_name)).expect("could not create log file"); // TODO: handle error

        // TODO: symlink logs to local project

        let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
        let level = if matches.get_flag("quiet") {
            LevelFilter::Off
        } else {
            match matches.get_count("verbose") {
                1 => LevelFilter::Error,
                2 => LevelFilter::Warn,
                3 => LevelFilter::Info,
                4 => LevelFilter::Debug,
                5 => LevelFilter::Trace,
                _ => unreachable!("invalid verbose level"),
            }
        };

        let config_term: simplelog::Config = simplelog::ConfigBuilder::new()
            .set_time_offset_to_local()
            .unwrap_or_else(|old| old)
            .set_level_color(Level::Error, Some(Color::Red))
            .set_level_color(Level::Warn, Some(Color::Yellow))
            .set_level_color(Level::Info, Some(Color::Green))
            .set_level_color(Level::Debug, Some(Color::Cyan))
            .set_level_color(Level::Trace, Some(Color::Magenta))
            .build();

        let config_write: simplelog::Config = simplelog::ConfigBuilder::new()
            .set_time_offset_to_local()
            .unwrap_or_else(|old| old)
            .build();

        if !matches.get_flag("quiet") {
            loggers.push(TermLogger::new(
                level,
                config_term,
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ));
        }
        loggers.push(WriteLogger::new(LevelFilter::Trace, config_write, f));
        loggers
    }) {
        log::warn!("Failed to set up logger: {}", e);
    };

    panic::set_hook(Box::new(|info| {
        log::error!("{}", info);
    }));

    match matches.subcommand() {
        Some(("build", m)) => build::run(m),
        #[cfg(feature = "shell-completions")]
        Some(("completions", m)) => completions::run(m),
        Some(("explain", m)) => explain::run(m),
        #[cfg(feature = "export")]
        Some(("export", m)) => export::run(m),
        Some(("health", m)) => health::run(m),
        Some(("init", m)) => init::run(m),
        Some(("logs", m)) => logs::run(m),
        #[cfg(feature = "manual")]
        Some(("manual", m)) => manual::run(m),
        #[cfg(feature = "config-schema")]
        Some(("schema", m)) => schema::run(m),
        #[cfg(feature = "share")]
        Some(("share", m)) => share::run(m),
        Some(("sync", m)) => sync::run(m),
        Some(("uuid", m)) => uuid::run(m),
        #[cfg(feature = "watch")]
        Some(("watch", m)) => watch::run(m),
        _ => unreachable!(),
    }
}
