use prettytable::Table;

use crate::build::uuidgen;
use crate::configuration::config::Config;

pub(crate) struct Environment {
    /// The configuration used for the current project.
    pub config: Option<Config>,

    /// The UUIDs stored for this project.
    pub uuids: Option<Table>,

    /// Whether the add-ons are beeing built in development or release mode.
    /// This is only set when the the `build` command is invoked.
    pub development: Option<bool>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            config: Config::try_parse(),
            uuids: uuidgen::read_uuids(),
            development: None,
        }
    }
}
