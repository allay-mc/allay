//! This module ensures the project can still be built and repairs the project
//! on corruption such when an internal file has been deleted.
//!
//! - `(repairing)` --- creating data by possibly changing previous data
//! - `(creating)` --- creating data without changing previous data

use std::fs;

use crate::{build::uuidgen, paths};

pub(crate) fn repair() -> std::io::Result<()> {
    let mut fixed: usize = 0;
    if paths::root().join(paths::internal()).is_dir() {
        log::debug!("(fine) internal directory exists");
    } else {
        log::warn!("(repairing) internal directory does not exist");
        fs::create_dir(paths::root().join(paths::internal()))?;
        fixed += 1;
    }
    if paths::root().join(paths::uuids()).is_file() {
        log::debug!("(fine) UUIDs are stored");
    } else {
        log::warn!("(repairing) UUIDs are not stored");
        uuidgen::save_uuids(&uuidgen::new())?;
        fixed += 1;
    }
    if paths::root().join(paths::build()).is_dir() {
        log::debug!("(fine) build directory exists");
    } else {
        log::info!("(creating) build directory does not exist");
        fs::create_dir(paths::root().join(paths::build()))?;
        fixed += 1;
    }
    if paths::root().join(paths::prebuild()).is_dir() {
        log::debug!("(fine) prebuild directory exists");
    } else {
        log::info!("(creating) prebuild directory does not exist");
        fs::create_dir(paths::root().join(paths::prebuild()))?;
        fixed += 1;
    }
    log::info!("fixed {} issue(s)", fixed);
    Ok(())
}
