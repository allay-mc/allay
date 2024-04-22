use std::env::current_dir;
use std::path::PathBuf;

/// The name of the file which contains the ID of the Allay project.
pub const FINGERPRINT: &'static str = ".allay-fingerprint";

/// Returns the path of the project root or [`None`] if an `allay.toml` file cannot be found.
pub fn try_root() -> Option<PathBuf> {
    let mut now = current_dir().ok()?;
    while !now.join("allay.toml").is_file() {
        now = now.parent()?.to_path_buf();
    }
    Some(now)
}

/// Returns the path of the project root.
///
/// # Panics
///
/// Panics when an `allay.toml` file cannot be found.
pub fn root() -> PathBuf {
    try_root().expect("expected `allay.toml` file in this or any parent directory")
}

// NOTE: The functions below should be `.join`ed with the result of the `root` or `try_root` function
//       to access the path relative to the project root instead of the directory the user is currently
//       in (e.g. `try_root()?.join(internal())`).

/// Returns the path of the internal directory where things like UUIDs are stored (`.allay/`).
pub fn internal() -> PathBuf {
    PathBuf::from(".allay")
}

pub fn global_internal() -> PathBuf {
    dirs::config_local_dir()
        .expect("no config directory found for this os")
        .join("allay")
}

pub fn logs() -> PathBuf {
    global_internal().join("logs")
}

pub fn project_id() -> PathBuf {
    internal().join("project_id.txt")
}

pub fn version() -> PathBuf {
    internal().join("version.txt")
}

/// Returns the path of the built add-on.
pub fn build() -> PathBuf {
    PathBuf::from("build.mcaddon")
}

pub fn uuids() -> PathBuf {
    internal().join("uuids.toml")
}

pub fn src() -> PathBuf {
    PathBuf::from("src")
}

pub fn src_bp() -> PathBuf {
    src().join("BP")
}

pub fn src_rp() -> PathBuf {
    src().join("RP")
}

pub fn src_sp() -> PathBuf {
    src().join("SP")
}

pub fn src_wt() -> PathBuf {
    src().join("WT")
}

pub fn config() -> PathBuf {
    PathBuf::from("allay.toml")
}

pub fn pack_icon() -> PathBuf {
    PathBuf::from("pack_icon.png")
}

pub fn gitignore() -> PathBuf {
    PathBuf::from(".gitignore")
}
