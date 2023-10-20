use clap::{Arg, ArgMatches, Command};

use crate::build::build;
use crate::environment::Environment;

pub(crate) fn cmd() -> Command {
    Command::new("build")
        .about("Builds the add-on")
        .visible_alias("b")
        .arg(
            Arg::new("file-type-bundle")
                .long("bundle")
                .help("Builds a `.mcaddon` file which can be used to import all components at once")
                .conflicts_with_all(["file-type-dir", "file-type-individual"])
                .num_args(0)
        )
        .arg(
            Arg::new("file-type-dir")
                .long("dir")
                .help("Builds the add-ons without zipping them which is useful for debugging for example to check whether all files were converted correctly")
                .conflicts_with_all(["file-type-bundle", "file-type-individual"])
                .num_args(0)
        )
        .arg(
            Arg::new("file-type-individual")
                .long("individual")
                .help("Build `.mcpack` or `.mctemplate` files which can be used to import individual components")
                .conflicts_with_all(["file-type-bundle", "file-type-dir"])
                .num_args(0)
        )
        .arg(
            Arg::new("release")
                .short('r')
                .long("release")
                .help("Builds the add-on in release mode")
                .num_args(0)
        )
}

pub(crate) fn run(matches: &ArgMatches, env: &mut Environment) {
    if env.config.is_none() {
        panic!("you are not in an allay project; initialize one with `allay init`");
    }
    env.development =
        Some(!matches.get_flag("release") || env.config.as_ref().unwrap().project.development);

    // TODO: infer file type, use --bundle, --dir or --individual or bundle if release mode,
    //       otherwise dir

    build(env).expect("failed to build project; try executing `allay repair`");
    log::info!("successfully built project");
}
