//! External resources used by Allay.
#![cfg(feature = "git")]

use std::{env, fs, io::BufReader, iter, path::Path};

use clap::builder::OsStr;
use git2::{Repository, RepositoryOpenFlags};
use crate::utils;

pub(crate) struct MinecraftVersion {
    pub(crate) version: String,
    pub(crate) date: String,
    pub(crate) is_latest: bool,
}

/// Returns versions of Minecraft.
pub(crate) fn minecraft_versions() -> Option<Vec<MinecraftVersion>> {
    let repo = match env::var_os("BEDROCK_SAMPLES")
        .map(repo_from_path)
        .unwrap_or_else(clone_bedrock_samples)
    {
        Ok(repo) => repo,
        Err(error) => {
            log::error!("Failed to get minecraft versions: {}", error);
            return None;
        }
    };
    log::info!("Pulling latest commit for minecraft versions");
    if let Err(error) = utils::git::pull(&repo) {
        log::error!("Failed to pull repository: {}", error);
    }
    let root_path = match repo.path().parent() {
        Some(root_path) => root_path,
        None => {
            log::error!("Failed to get root path of bedrock-samples");
            return None;
        }
    };
    let data_path = root_path.join("version.json");
    let file = match fs::File::open(&data_path) {
        Ok(file) => file,
        Err(error) => {
            log::error!(
                "Error while trying to open {}: {}",
                data_path.display(),
                error
            );
            return None;
        }
    };
    let reader = BufReader::new(file);
    let data: serde_json::Value = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(error) => {
            log::error!(
                "version.json at {} contains invalid JSON: {}",
                data_path.display(),
                error
            );
            return None;
        }
    };
    let object = match data.as_object() {
        Some(object) => object,
        None => {
            log::error!(
                "version.json at {} has an invalid format",
                data_path.display()
            );
            return None;
        }
    };
    let mut versions: Vec<MinecraftVersion> = Vec::new();
    for (key, value) in object {
        let is_latest = key == "latest";
        let version_and_date = match value.as_object() {
            Some(version_and_date) => version_and_date,
            None => {
                log::error!(
                    "version.json at {} has an invalid format",
                    data_path.display()
                );
                return None;
            }
        };
        let version = match version_and_date.get("version") {
            Some(version) => version,
            None => {
                log::error!(
                    "version.json at {} has an invalid format",
                    data_path.display()
                );
                return None;
            }
        };
        let version = match version.as_str() {
            Some(val) => val,
            None => {
                log::error!(
                    "version.json at {} has an invalid format",
                    data_path.display()
                );
                return None;
            }
        };
        let date = match version_and_date.get("date") {
            Some(date) => date,
            None => {
                log::error!(
                    "version.json at {} has an invalid format",
                    data_path.display()
                );
                return None;
            }
        };
        let date = match date.as_str() {
            Some(val) => val,
            None => {
                log::error!(
                    "version.json at {} has an invalid format",
                    data_path.display()
                );
                return None;
            }
        };
        let mcver = MinecraftVersion {
            version: version.to_string(),
            date: date.to_string(),
            is_latest,
        };
        if !versions.iter().any(|v| v.version == mcver.version) {
            versions.push(mcver);
        }
    }
    Some(versions)
}

pub(crate) fn clone_bedrock_samples() -> Result<Repository, git2::Error> {
    let dest = allay::paths::global::db().join("bedrock-samples");
    repo_from_path(&dest).or_else(|_| {
        log::info!("Downloading bedrock samples; this may take a while");
        Repository::clone("https://github.com/Mojang/bedrock-samples.git", &dest)
    })
}

fn repo_from_path<P>(path: P) -> Result<Repository, git2::Error>
where
    P: AsRef<Path>,
{
    Repository::open_ext(path, RepositoryOpenFlags::NO_SEARCH, iter::empty::<OsStr>())
}
