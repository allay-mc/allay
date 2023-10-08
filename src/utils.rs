pub(crate) use std::path::Path;

pub(crate) use clap::crate_version;

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
