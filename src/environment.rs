use std::fs;
use std::str;

use prettytable::Table;

use crate::build::uuidgen;
use crate::configuration::config::Config;
use crate::paths;

pub(crate) struct Environment {
    /// The configuration used for the current project.
    pub config: Config,

    /// The UUIDs stored for this project.
    pub uuids: Table,

    /// Whether the add-ons are beeing built in development or release mode.
    pub development: bool,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            config: toml::from_str(
                &str::from_utf8(&fs::read(paths::config()).expect("cannot read allay.toml"))
                    .expect("allay.toml is not UTF-8"),
            )
            .expect("allay.toml is invalid TOML or has an invalid field"),
            uuids: uuidgen::read_uuids().expect("error reading UUIDs"),
            development: true,
        }
    }
}
