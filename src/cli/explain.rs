use allay::diagnostic::{self, Diagnostic};
use clap::{Arg, ArgMatches, Command};
use std::process::ExitCode;

pub fn cmd() -> Command {
    Command::new("explain")
        .about("Explain an error or a warning")
        .arg(Arg::new("id").help("The ID of the diagnostic (e.g. `W0001` or `E0001`)"))
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let id: &String = matches.get_one("id").unwrap();
    if id.starts_with(diagnostic::ERROR_PREFIX) {
        // NOTE: there are no error codes yet
        ExitCode::FAILURE
    } else if id.starts_with(diagnostic::WARNING_PREFIX) {
        let code: u8 = id
            .strip_prefix(diagnostic::WARNING_PREFIX)
            .unwrap()
            .parse()
            .expect("Code is invalid (must be 0-255)");
        let w = diagnostic::Warning::from_code(code);
        match w.extensive_description() {
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
    } else {
        log::error!(
            "Invalid ID: it must either start with `{}` or `{}`",
            diagnostic::ERROR_PREFIX,
            diagnostic::WARNING_PREFIX
        );
        ExitCode::FAILURE
    }
}
