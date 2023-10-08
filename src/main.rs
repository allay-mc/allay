#![doc = include_str!("../README.md")]

pub(crate) mod addon;
pub(crate) mod build;
pub(crate) mod cli;
pub(crate) mod configuration;
pub(crate) mod environment;
pub(crate) mod health;
pub(crate) mod paths;
pub(crate) mod scripts;
pub(crate) mod template;
pub(crate) mod utils;

// TODO: raise on keys not present in `Config`

use environment::Environment;
// use std::panic;

fn main() {
    /*
    panic::set_hook(Box::new(|info| {
        log::error!(
            "allay exited unsuccesfully{} - {}",
            match info.location() {
                Some(l @ std::panic::Location { .. }) =>
                    format!(": {}:{}:{}", l.file(), l.line(), l.column()),
                None => String::from(""),
            },
            info.message()
        );
    }));
    */
    let env = &mut Environment::new();
    let matches = cli::cmd().get_matches();
    cli::run(&matches, env);
}
