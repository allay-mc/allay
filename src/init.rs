//! Globally initialze Allay.
//!
//! This is an important step when Allay is used the first time as it creates directories used for saving
//! log files for example. This should also be run when such directories are deleted by the user.

use allay::paths;
use std::fs;
use std::io::ErrorKind;

pub fn init() {
    // in some scenarios the config directory is not present so we create it if neccessary
    let _ =
        fs::create_dir(dirs::config_local_dir().expect("no config directory found for this os"));

    if let Err(e) = fs::create_dir(paths::global_internal()) {
        if e.kind() != ErrorKind::AlreadyExists {
            eprintln!("Warning: could not create global internal directory: {}; This might lead to unexpected errors", e);
        }
    };
    if let Err(e) = fs::create_dir(paths::logs()) {
        if e.kind() != ErrorKind::AlreadyExists {
            eprintln!(
               "Warning: could not create logs directory: {}; This might lead to unexpected errors",
                e
            );
        }
    };
}
