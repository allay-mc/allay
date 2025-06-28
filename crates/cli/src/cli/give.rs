// TODO: advance easter egg: if user gives expensive item something interesting happens...

use clap::{Arg, ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("give")
        .about("Give Allay something")
        .arg(Arg::new("object").help("The object to give").required(true))
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let object: &String = matches.get_one("object").unwrap();
    println!("Let's see if I find {} somewhere around here...", object);
    ExitCode::SUCCESS
}
