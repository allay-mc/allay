//! Utilities for the state of the project.

use super::{Profile, Project};
use crate::Pack;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{cmp, fs, io, path::Path};
use uuid::Uuid;

/// Generates a new ID for a project.
pub fn generate_project_id() -> super::Id {
    Uuid::new_v4()
}

struct Greatest<T> {
    value: Option<T>,
}

impl<T: cmp::PartialOrd> Greatest<T> {
    /// Updates the value with `value` only if the value is not yet set or if it is greater.
    pub fn update(&mut self, value: T) {
        if self
            .value
            .as_ref()
            .is_none_or(|old_value| *old_value < value)
        {
            self.value = Some(value);
        }
    }

    /// Returns the contained value.
    pub fn get(self) -> Option<T> {
        self.value
    }
}

impl<T> Default for Greatest<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

/// Returns the last modification of any file or directory within a directory or the directory
/// itself.
fn dir_last_change(dir: &Path) -> io::Result<Option<SystemTime>> {
    let mut recent_modification: Greatest<SystemTime> = Greatest::default();

    match dir.metadata().and_then(|metadata| metadata.modified()) {
        Ok(modified) => recent_modification.update(modified),
        Err(error) => {
            log::error!("{}", error);
        }
    };

    for entry in dir.read_dir()? {
        let entry = entry?.path();
        log::trace!("Checking last modification of {}", entry.display());
        match entry.metadata().and_then(|metadata| metadata.modified()) {
            Ok(modified) => recent_modification.update(modified),
            Err(error) => {
                log::error!("{}", error);
                continue;
            }
        };
        if entry.is_dir() {
            match dir_last_change(&entry)? {
                Some(modified) => recent_modification.update(modified),
                None => {
                    log::trace!("Cannot determine last modification of {}", entry.display());
                    continue;
                }
            };
        }
    }

    Ok(recent_modification.get())
}

/// Returns `true` when the provided pack kind contains at least one file or directory.
pub fn source_has_content(Project { root, .. }: &Project, pack: &Pack) -> bool {
    let path = pack.path_source(root);
    path.is_dir() && path.read_dir().is_ok_and(|mut r| r.next().is_some())
}

/// Returns `true` when the source has changed since the last (successful) build.
///
/// If the time cannot be dertemined, `true` is returned as well.
pub fn source_has_changed(Project { root, config, .. }: &Project, profile: Profile) -> bool {
    // TODO: see also: watch/poll.rs
    let mut significant_paths: Vec<PathBuf> = vec![
        crate::paths::project::source(root),
        crate::paths::project::config(root),
        crate::paths::project::pack_icon(root),
    ];
    for path in &config.build.extra_watch {
        significant_paths.push(path.to_path_buf());
    }

    // TODO: `config.build.ignore_watch`

    let last_build_path = crate::paths::project::last_build_for_profile(root, profile);

    let mut last_modification: SystemTime = SystemTime::UNIX_EPOCH;

    for path in significant_paths {
        last_modification = cmp::max(
            last_modification,
            if !path.exists() {
                log::warn!(
                    "Path `{}` listed in `extra-watch` does not exist",
                    path.display()
                );
                last_modification
            } else if path.is_dir() {
                match dir_last_change(&path) {
                    Ok(Some(modified)) => modified,
                    Ok(None) => {
                        log::trace!("Cannot determine last modification of {}", path.display());
                        return true;
                    }
                    Err(error) => {
                        log::error!(
                            "Cannot determine last modification of `{}`: {}",
                            path.display(),
                            error
                        );
                        return true;
                    }
                }
            } else {
                match fs::metadata(&path).and_then(|metadata| metadata.modified()) {
                    Ok(modified) => modified,
                    Err(error) => {
                        log::error!("{}", error);
                        return true;
                    }
                }
            },
        );
    }

    let last_build = match fs::metadata(last_build_path).and_then(|metadata| metadata.modified()) {
        Ok(modified) => modified,
        Err(error) => {
            log::debug!("Error while retrieving time of last build: {}. This is normal for the project's first build", error);
            return true;
        }
    };

    log::trace!(
        "Last modification of signicant paths: {}",
        time::OffsetDateTime::from(last_modification)
    );
    log::trace!("Last build: {}", time::OffsetDateTime::from(last_build));

    last_modification.duration_since(last_build).is_ok()
}

/// Creates/overrides the file containing the project id.
pub fn fix_missing_project_id(root: &Path) -> io::Result<()> {
    fs::write(
        crate::paths::project::project_id(root),
        generate_project_id().as_bytes(),
    )
}

/// Returns `true` when there is a debug build of the current project.
pub fn has_debug_build(Project { root, .. }: &Project) -> bool {
    crate::paths::project::build_file_debug(root).exists()
}

/// Returns `false` when there is a release build of the current project.
pub fn has_release_build(Project { root, .. }: &Project) -> bool {
    crate::paths::project::build_file_release(root).exists()
}

