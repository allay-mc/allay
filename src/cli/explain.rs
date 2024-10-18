use crate::diagnostic::{self, Diagnostic};
use clap::{Arg, ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("explain")
        .about("Explain an error or a warning")
        .arg(Arg::new("id").help("The ID of the diagnostic without the prefix (e.g. `0001`)"))
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let id: &String = matches.get_one("id").unwrap();
    let code: u8 = id
        .parse()
        .unwrap_or_else(|_| { panic!("{}", "Code is invalid (must be 0-255)".to_string()) });
    let notif = match diagnostic::Notification::from_code(code) {
        Some(n) => n,
        None => {
            log::error!("Error or warning with code {} does not exist", code);
            return ExitCode::FAILURE;
        }
    };
    match notif.extensive_description() {
        Some(desc) => {
            // TODO: open in pager
            // if open::with(desc, std::env::var("PAGER").unwrap_or(String::from("less"))).is_err()
            // {
            println!("{}", desc);
            // };
            ExitCode::SUCCESS
        }
        None => {
            log::error!("Warning exists but does not conatin an extensive description");
            ExitCode::FAILURE
        }
    }
}
