//! Tree generated when an Allay project is initilized.
//! The files are copied from the `files` directory into
//! the specified path.

// TODO: use crate::paths

use crate::paths;
use std::fs;
use std::path::Path;

pub(crate) fn create_template(path: &Path) -> std::io::Result<()> {
    fs::create_dir(path.join(paths::internal()))?;
    fs::create_dir(path.join(paths::prebuild()))?;
    fs::create_dir(path.join(paths::build()))?;
    fs::create_dir(path.join(paths::src()))?;
    fs::create_dir(path.join(paths::src_bp()))?;
    fs::create_dir(path.join(paths::src_rp()))?;
    fs::create_dir(path.join(paths::src_sp()))?;
    fs::create_dir(path.join(paths::src_wt()))?;
    fs::write(
        path.join(paths::gitignore()),
        include_bytes!("files/gitignore"),
    )?;
    fs::write(
        path.join(paths::config()),
        include_bytes!("files/allay.toml"),
    )?;
    fs::write(path.join("README.md"), include_bytes!("files/README.md"))?;
    Ok(())
}
