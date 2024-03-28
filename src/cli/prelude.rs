use clap::{Arg, ArgAction, Command};

pub trait CommandExt: Sized {
    fn _arg(self, arg: Arg) -> Self;

    fn arg_release(self) -> Self {
        self._arg(
            Arg::new("build-release")
                .short('r')
                .long("release")
                .help("Builds the project in release mode")
                .action(ArgAction::SetTrue)
                .conflicts_with("build-debug"),
        )
    }

    fn arg_debug(self) -> Self {
        self._arg(
            Arg::new("build-debug")
                .short('d')
                .long("debug")
                .help("Builds the project in debug mode")
                .action(ArgAction::SetTrue)
                .conflicts_with("build-release"),
        )
    }

    fn arg_build_mode(self) -> Self {
        self.arg_debug().arg_release()
    }

    fn arg_build_opts(self) -> Self {
        self.arg_build_mode()
    }
}

impl CommandExt for Command {
    fn _arg(self, arg: Arg) -> Self {
        self.arg(arg)
    }
}
