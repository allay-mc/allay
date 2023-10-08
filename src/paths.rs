use std::env::current_dir;
use std::path::PathBuf;

pub(crate) fn try_root() -> Option<PathBuf> {
    let mut now = current_dir().expect("cannot get current directory");
    while !now.join("allay.toml").is_file() {
        now = now.parent()?.to_path_buf();
    }
    Some(now)
}

pub(crate) fn root() -> PathBuf {
    try_root().expect("expected `allay.toml` file in this or any parent directory")
}

pub(crate) fn internal() -> PathBuf {
    PathBuf::from(".allay")
}

pub(crate) fn prebuild() -> PathBuf {
    internal().join("prebuild")
}

pub(crate) fn prebuild_bp() -> PathBuf {
    prebuild().join("BP")
}

pub(crate) fn prebuild_rp() -> PathBuf {
    prebuild().join("RP")
}

pub(crate) fn prebuild_sp() -> PathBuf {
    prebuild().join("SP")
}

pub(crate) fn prebuild_wt() -> PathBuf {
    prebuild().join("WT")
}

pub(crate) fn build() -> PathBuf {
    PathBuf::from("build")
}

pub(crate) fn build_bp() -> PathBuf {
    build().join("BP")
}

pub(crate) fn build_rp() -> PathBuf {
    build().join("RP")
}

pub(crate) fn build_sp() -> PathBuf {
    build().join("SP")
}

pub(crate) fn build_wt() -> PathBuf {
    build().join("WT")
}

pub(crate) fn uuids() -> PathBuf {
    internal().join("uuids.csv")
}

pub(crate) fn src() -> PathBuf {
    PathBuf::from("src")
}

pub(crate) fn src_bp() -> PathBuf {
    src().join("BP")
}

pub(crate) fn src_rp() -> PathBuf {
    src().join("RP")
}

pub(crate) fn src_sp() -> PathBuf {
    src().join("SP")
}

pub(crate) fn src_wt() -> PathBuf {
    src().join("WT")
}

pub(crate) fn config() -> PathBuf {
    PathBuf::from("allay.toml")
}

pub(crate) fn pack_icon() -> PathBuf {
    PathBuf::from("icon.png")
}

pub(crate) fn gitignore() -> PathBuf {
    PathBuf::from(".gitignore")
}
