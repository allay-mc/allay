use clap::{Arg, ArgAction, ArgMatches, Command};
use std::process;
use supports_color::Stream;
use textwrap_macros::dedent;

mod build;
mod complete;
mod create;
mod docs;
mod eval;
mod ext;
mod give;
mod share;
mod uuid;
mod watch;

pub(crate) fn cmd() -> Command {
    Command::new("allay")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(if supports_color::on(Stream::Stdout).is_some() {
            concat!(
                "\x1b[46m \x1b[47m \x1b[46m \x1b[47m \x1b[46m \x1b[0m", // Allay ANSI art
                " ",
                clap::crate_description!()
            )
        } else {
            clap::crate_description!()
        })
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .help("Enables logging messages (repeat to increase verbosity)")
                .long_help(dedent!(
                    "
                    Enables logging messages

                    -v      errors
                    -vv     + warnings
                    -vvv    + info (default)
                    -vvvv   + debug
                    -vvvvv  + trace

                    Use `--quiet` to disable logging completely
                "
                ))
                .action(clap::ArgAction::Count)
                .value_parser(clap::value_parser!(u8).range(1..=5))
                .default_value("3"),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .help("Disables logging completely")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("verbose")
                .global(true),
        )
        .arg(
            Arg::new("license")
                .long("license")
                .help("Displays the license of Allay")
                .action(ArgAction::SetTrue),
        )
        .subcommands([
            build::cmd(),
            #[cfg(feature = "completions")]
            complete::cmd(),
            create::cmd(),
            #[cfg(feature = "manual")]
            docs::cmd(),
            eval::cmd(),
            give::cmd(),
            #[cfg(feature = "share")]
            share::cmd(),
            uuid::cmd(),
            #[cfg(feature = "watch")]
            watch::cmd(),
        ])
}

pub(crate) fn run(matches: &ArgMatches) -> process::ExitCode {
    if matches.get_flag("license") {
        println!("{}", include_str!("../../../../LICENSE.txt"));
        return process::ExitCode::SUCCESS;
    }
    match matches.subcommand() {
        Some(("build", m)) => build::run(m),
        #[cfg(feature = "completions")]
        Some(("complete", m)) => complete::run(m),
        Some(("create", m)) => create::run(m),
        #[cfg(feature = "manual")]
        Some(("docs", m)) => docs::run(m),
        Some(("eval", m)) => eval::run(m),
        Some(("give", m)) => give::run(m),
        #[cfg(feature = "share")]
        Some(("share", m)) => share::run(m),
        Some(("uuid", m)) => uuid::run(m),
        #[cfg(feature = "watch")]
        Some(("watch", m)) => watch::run(m),
        Some((name, _)) => unreachable!("no case for subcommand {}", name),
        None => unreachable!("help should be displayed"),
    }
}
