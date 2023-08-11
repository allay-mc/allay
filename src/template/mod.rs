//! Tree generated when an Allay project is initilized.
//! The files are copied from the `files` directory into
//! the specified path.

// TODO: use crate::paths

use std::fs;
use std::path::Path;

pub(crate) fn create_template(path: &Path) -> std::io::Result<()> {
    fs::create_dir(path.join(".allay"))?;
    fs::create_dir(path.join(".allay/prebuild/"))?;
    fs::create_dir(path.join("build"))?;
    fs::create_dir(path.join("scripts"))?;
    fs::create_dir(path.join("src"))?;
    fs::create_dir(path.join("src/BP"))?;
    fs::create_dir(path.join("src/RP"))?;
    fs::create_dir(path.join("src/SP"))?;
    fs::create_dir(path.join("src/WT"))?;
    fs::write(path.join(".gitignore"), include_bytes!("files/gitignore"))?;
    fs::write(path.join("allay.toml"), include_bytes!("files/allay.toml"))?;
    fs::write(path.join("README.md"), include_bytes!("files/README.md"))?;
    Ok(())
}
