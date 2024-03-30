use crate::Config;
use clap::{Arg, ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("schema")
        .about("Print the JSON schema for the configuration file")
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty prints the JSON")
                .num_args(0),
        )
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let pretty = matches.get_flag("pretty");
    let schema = schemars::schema_for!(Config);
    println!(
        "{}",
        {
            if pretty {
                serde_json::to_string_pretty
            } else {
                serde_json::to_string
            }
        }(&schema)
        .unwrap()
    );

    ExitCode::SUCCESS
}
