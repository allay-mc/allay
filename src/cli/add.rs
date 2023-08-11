use clap::{Arg, ArgMatches, Command};
use std::fs;
use std::path::PathBuf;

pub(crate) fn cmd() -> Command {
    Command::new("add")
        .about("Download and add an official script")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Specify the path to copy the script to")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("scripts"),
        )
        .arg(
            Arg::new("scripts")
                .help("The name of the script to load without the file extension")
                .required(true)
                // TODO: long help with example
                .num_args(1..),
        )
}

pub(crate) fn run(matches: &ArgMatches) {
    let path: &PathBuf = matches.get_one("path").unwrap();
    let scripts: Vec<_> = matches.get_many::<String>("scripts").unwrap().collect();
    for script in scripts {
        let response = minreq::get(format!(
            "https://raw.githubusercontent.com/allay-mc/scripts/main/{}.rb",
            script,
        ))
        .send()
        .expect("cannot request URL");
        let data = response.as_bytes();
        // TODO: skip with warning if script already exists
        fs::write(path.join(script), data).expect("cannot write file");
    }
    // TODO: refer it in allay.toml?
}
