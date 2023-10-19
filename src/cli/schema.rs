use crate::environment::Environment;
use clap::{ArgMatches, Command};

pub(crate) fn cmd() -> Command {
    Command::new("schema").about("Prints the schema of the configuration file")
}

pub(crate) fn run(_matches: &ArgMatches, _env: &mut Environment) {
    let schema = schemars::schema_for!(crate::configuration::config::Config);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
