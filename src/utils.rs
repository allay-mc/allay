pub(crate) use std::path::Path;

use anyhow::{anyhow, Context};
pub(crate) use clap::crate_version;

use crate::configuration::manifest::BaseGameVersion;

/// Returns `true` when the specified directory is empty.
/// Returns an [Err] if
///
/// - the provided `path` doesn't exist,
/// - the process lacks permission to view the contents or
/// - the `path` points at non-directory file.
pub(crate) fn empty_dir(path: &Path) -> std::io::Result<bool> {
    Ok(path.read_dir()?.next().is_none())
}

pub(crate) fn version_as_array(version: &str) -> anyhow::Result<(usize, usize, usize)> {
    let mut parsed_parts: Vec<usize> = Vec::new();
    let parts: Vec<&str> = version.split('.').collect();
    for part in parts {
        let parsed_part = part.parse().with_context(|| "expected an integer")?;
        parsed_parts.push(parsed_part);
    }
    let mut parsed = parsed_parts.into_iter();
    Ok((
        parsed.next().with_context(|| "missing number")?,
        parsed.next().with_context(|| "missing number")?,
        parsed.next().with_context(|| "missing number")?,
    ))
}

pub(crate) fn version_as_array_or_wild(version: &str) -> anyhow::Result<BaseGameVersion> {
    match version_as_array(version) {
        Ok(v) => Ok(BaseGameVersion::Version(v.0, v.1, v.2)),
        Err(_) if version == "*" => Ok(BaseGameVersion::Wild),
        Err(_) => Err(anyhow!("expected `x.y.z` or `*`")),
    }
}
