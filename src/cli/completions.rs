use clap::{Arg, ArgAction, ArgMatches, Command};
use clap_complete::Shell;
use std::{io, process::ExitCode};

pub fn cmd() -> Command {
    Command::new("completions")
        .about("Generate shell completions")
        .arg(
            Arg::new("shell")
                .help("The shell for which to generate completions")
                .required(true)
                .value_parser(clap::value_parser!(Shell)),
        )
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let generator = matches.get_one::<Shell>("shell").copied().unwrap();
    let mut command = super::cmd();
    print_completions(generator, &mut command);

    ExitCode::SUCCESS
}

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
