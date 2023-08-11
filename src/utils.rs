use std::{fs, path::Path};

pub(crate) use clap::{crate_authors, crate_description, crate_name, crate_version};

use crate::paths;

/// Returns `true` when the specified directory is empty.
/// Returns an [Err] if
///
/// - the provided `path` doesn't exist,
/// - the process lacks permission to view the contents or
/// - the `path` points at non-directory file.
pub(crate) fn empty_dir(path: &Path) -> std::io::Result<bool> {
    Ok(path.read_dir()?.next().is_none())
}

pub(crate) fn version_as_array(version: &str) -> (usize, usize, usize) {
    let parts: Vec<usize> = version
        .split('.')
        .map(|x| x.parse().expect("invalid version format"))
        .collect();
    (parts[0], parts[1], parts[2])
}

/// Returns `true` when the project contains data within the `BP` directory.
pub(crate) fn has_bp() -> bool {
    !empty_dir(&paths::src_bp()).unwrap_or(true)
}

/// Returns `true` when the project contains data within the `RP` directory.
pub(crate) fn has_rp() -> bool {
    !empty_dir(&paths::src_rp()).unwrap_or(true)
}

/// Returns `true` when the project contains data within the `SP` directory.
pub(crate) fn has_sp() -> bool {
    !empty_dir(&paths::src_sp()).unwrap_or(true)
}

/// Returns `true` when the project contains data within the `WT` directory.
pub(crate) fn has_wt() -> bool {
    !empty_dir(&paths::src_wt()).unwrap_or(true)
}
