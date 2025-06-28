//! Command extensions.

use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

pub trait CommandExt: Sized {
    fn _arg(self, arg: Arg) -> Self;

    fn arg_plugins(self) -> Self {
        self._arg(
            Arg::new("skip-plugins")
                .long("skip")
                .help("Skip running a plugin")
                .action(ArgAction::Append),
        )
    }

    fn arg_sync(self) -> Self {
        self._arg(
            Arg::new("build-enable-sync")
                .long("sync")
                .help("Synchronize build. This flag has precedence over the `sync` field in the `[build]` section of `allay.toml`.")
                .action(ArgAction::SetTrue),
        )
        ._arg(
            Arg::new("build-disable-sync")
                .long("no-sync")
                .help("Don't synchronize build. This flag has precedence over the `sync` field in the `[build]` section of `allay.toml`.")
                .action(ArgAction::SetTrue)
                .conflicts_with("build-enable-sync"),
        )
    }

    fn arg_release(self) -> Self {
        self._arg(
            Arg::new("build-release")
                .long("release")
                .help("Build the project in release mode")
                .long_help("Build the project in release mode. This flag has precedence over the `debug` field in the `[build]` section of `allay.toml`.")
                .action(ArgAction::SetTrue)
                .conflicts_with("build-debug"),
        )
    }

    fn arg_debug(self) -> Self {
        self._arg(
            Arg::new("build-debug")
                .long("debug")
                .help("Build the project in debug mode")
                .long_help("Build the project in debug mode. This flag has precedence over the `debug` field in the `[build]` section of `allay.toml`.")
                .action(ArgAction::SetTrue)
                .conflicts_with("build-release"),
        )
    }

    fn arg_force_build(self) -> Self {
        self._arg(
            Arg::new("build-force")
                .long("force")
                .help("Force a rebuild and ignore previous build")
                .action(ArgAction::SetTrue),
        )
    }

    fn arg_project_dir(self) -> Self {
        self._arg(
            Arg::new("project-dir")
                .short('d')
                .long("directory")
                .help("Specify the directory of the Allay project")
                .value_parser(clap::value_parser!(PathBuf)),
        )
    }

    /// Adds an argument for specifying the directory where the project should be initialized.
    fn arg_init_target(self) -> Self {
        self._arg(
            Arg::new("init-target")
                .short('d')
                .long("directory")
                .help("Specify the directory where the project should be initialized")
                .value_parser(clap::value_parser!(PathBuf)),
        )
    }

    fn arg_no_git(self) -> Self {
        self._arg(
            Arg::new("init-git")
                .long("no-git")
                .help("Prevent initialization of new git repository")
                .action(ArgAction::SetFalse),
        )
    }

    /// Adds arguments for specifying the build mode (debug/release).
    fn arg_build_mode(self) -> Self {
        self.arg_debug().arg_release()
    }

    /// Adds arguments for the build process.
    fn arg_build_opts(self) -> Self {
        self.arg_build_mode()
            .arg_project_dir()
            .arg_force_build()
            .arg_sync()
            .arg_plugins()
    }

    /// Adds argument for the initialization process.
    fn arg_init_opts(self) -> Self {
        self.arg_init_target().arg_no_git()
    }
}

impl CommandExt for Command {
    fn _arg(self, arg: Arg) -> Self {
        self.arg(arg)
    }
}
