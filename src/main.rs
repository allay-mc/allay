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

fn main() {
    let matches = cli::cmd().get_matches();
    cli::run(&matches);
}
