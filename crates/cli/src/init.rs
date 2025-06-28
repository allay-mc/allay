// NOTE: We make use of logging here but the logger is not configured at this point.

use crate::utils;
use include_dir::include_dir;
#[cfg(feature = "json-schema")]
use schemars::schema_for;
use std::{fs, io};

const STANDARD_TEMPLATES: include_dir::Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/src/templates/std");
const MANUAL: include_dir::Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../target/manual");

pub(crate) fn initialize() -> io::Result<()> {
    create_data_dir()?;
    create_logs_dir()?;
    create_templates_dir()?;
    create_manual_dir()?;
    create_db_dir()?;

    create_standard_templates()?;
    create_manual()?;
    #[cfg(feature = "json-schema")]
    create_schema()?;

    Ok(())
}

fn create_data_dir() -> io::Result<()> {
    log::debug!("Creating global config dir");
    fs::create_dir_all(allay::paths::global::data())?;
    Ok(())
}

fn create_db_dir() -> io::Result<()> {
    log::debug!("Creating database dir");
    fs::create_dir_all(allay::paths::global::db())
}

fn create_logs_dir() -> io::Result<()> {
    log::debug!("Creating global logs dir");
    fs::create_dir_all(allay::paths::global::logs())?;
    Ok(())
}

fn create_templates_dir() -> io::Result<()> {
    log::debug!("Creating global templates dir");
    fs::create_dir_all(allay::paths::global::templates())?;
    Ok(())
}

fn create_standard_templates() -> io::Result<()> {
    log::debug!("Creating standard templates");
    utils::fs::copy_included_dir(&STANDARD_TEMPLATES, &allay::paths::global::templates())
}

fn create_manual_dir() -> io::Result<()> {
    log::debug!("Creating global manual dir");
    fs::create_dir_all(allay::paths::global::manual())?;
    Ok(())
}

fn create_manual() -> io::Result<()> {
    log::debug!("Creating manual");
    // TODO: apply patch
    utils::fs::copy_included_dir(&MANUAL, &allay::paths::global::manual())?;
    fs::write(allay::paths::global::manual().join("version.txt"), allay::VERSION)?;
    Ok(())
}

#[cfg(feature = "json-schema")]
fn create_schema() -> io::Result<()> {
    log::debug!("Creating schema");
    let schema = schema_for!(allay::Config);
    fs::write(
        allay::paths::global::schema_version_specific(),
        serde_json::to_string(&schema).expect("Failed to create JSON schema from configuration"),
    )?;
    if !allay::paths::global::schema().exists() {
        #[cfg(unix)]
        std::os::unix::fs::symlink(
            allay::paths::global::schema_version_specific(),
            allay::paths::global::schema(),
        )?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(
            allay::paths::global::schema_version_specific(),
            allay::paths::global::schema(),
        )?;
    }
    Ok(())
}
