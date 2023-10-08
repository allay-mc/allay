use clap::{Arg, ArgMatches, Command};
use std::fs;
use std::path::PathBuf;

use crate::environment::Environment;

const HTTP_OK: i32 = 200;

pub(crate) fn cmd() -> Command {
    Command::new("add")
        .about("Download and add an official script")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Specify the path to copy the script to")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("scripts"), // TODO: base-path in config file
        )
        .arg(
            Arg::new("scripts")
                .help("The name of the script to load without the file extension")
                .required(true)
                // TODO: long help with example
                .num_args(1..),
        )
}

pub(crate) fn run(matches: &ArgMatches, _env: &mut Environment) {
    let path: &PathBuf = matches.get_one("path").unwrap();
    if !path.exists() {
        panic!("the path '{}' does not exist", path.display());
    }
    let scripts: Vec<_> = matches.get_many::<String>("scripts").unwrap().collect();
    for script in scripts {
        let response = minreq::get(format!(
            "https://raw.githubusercontent.com/allay-mc/scripts/master/{}.rb",
            script,
        ))
        .send()
        .expect("cannot request URL");
        if response.status_code != HTTP_OK {
            panic!("response was unsuccessful ({})", response.status_code);
        }
        let data = response.as_bytes();
        let script_path = path.join(script).with_extension("rb");
        if script_path.exists() {
            log::error!("file '{}' already exists; make sure you spelled the script name correctly and did't include the `.rb` file extension", script_path.display());
        } else {
            fs::write(script_path, data).expect("cannot write file");
            log::info!(
                "successfully added {}; remember to refer it in `allay.toml`",
                script
            );
        }
    }
}
