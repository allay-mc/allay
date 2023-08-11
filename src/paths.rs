use std::env::current_dir;
use std::path::PathBuf;

pub(crate) fn root() -> PathBuf {
    let mut now = current_dir().expect("cannot get current directory");
    while !now.join("allay.toml").is_file() {
        now = now
            .parent()
            .expect("expected `allay.toml` file in this or any parent directory")
            .to_path_buf();
    }
    now
}

pub(crate) fn internal() -> PathBuf {
    root().join(".allay")
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
    root().join("build")
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
    root().join("src")
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
    root().join("allay.toml")
}

pub(crate) fn pack_icon() -> PathBuf {
    root().join("pack_icon.png")
}

pub(crate) fn gitignore() -> PathBuf {
    root().join(".gitignore")
}
