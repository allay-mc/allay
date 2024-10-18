use clap::{builder::PossibleValue, Arg, ArgMatches, Command};
use clap_complete::Generator;
use std::{io, process::ExitCode};

pub fn cmd() -> Command {
    Command::new("completions")
        .about("Generate shell completions")
        .arg(
            Arg::new("shell")
                .help("The shell for which to generate completions")
                .required(true)
                .ignore_case(true)
                .value_parser(clap::value_parser!(Shell)),
        )
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let generator: Shell = matches.get_one::<Shell>("shell").cloned().unwrap();
    let mut command = super::cmd();
    print_completions(generator, &mut command);

    ExitCode::SUCCESS
}

#[derive(Clone, Copy, Debug)]
enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
    Nushell,
    Fig,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::shells::Bash.file_name(name),
            Self::Elvish => clap_complete::shells::Elvish.file_name(name),
            Self::Fish => clap_complete::shells::Fish.file_name(name),
            Self::PowerShell => clap_complete::shells::PowerShell.file_name(name),
            Self::Zsh => clap_complete::shells::Zsh.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::Fig => clap_complete_fig::Fig.file_name(name),
        }
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn io::Write) {
        match self {
            Self::Bash => clap_complete::shells::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::shells::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::shells::Fish.generate(cmd, buf),
            Self::PowerShell => clap_complete::shells::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::shells::Zsh.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::Fig => clap_complete_fig::Fig.generate(cmd, buf),
        }
    }
}

impl clap::ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
            Self::Nushell,
            Self::Fig,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Bash => Some(PossibleValue::new("bash")),
            Self::Elvish => Some(PossibleValue::new("elvish")),
            Self::Fish => Some(PossibleValue::new("fish")),
            Self::PowerShell => Some(PossibleValue::new("powershell")),
            Self::Zsh => Some(PossibleValue::new("zsh")),
            Self::Nushell => Some(PossibleValue::new("nushell")),
            Self::Fig => Some(PossibleValue::new("fig")),
        }
    }
}

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
