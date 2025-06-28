//! Common paths used by Allay.

/// Project-specific paths.
pub mod project {
    use crate::{project::Profile, Pack};
    use std::path::{Path, PathBuf};

    /// The path to the config file of a project.
    pub fn config(root: &Path) -> PathBuf {
        root.join("allay.toml")
    }

    /// The path to the lock file of a project.
    pub fn lock(root: &Path) -> PathBuf {
        root.join("allay.lock")
    }

    /// The path to the global pack icon of a project.
    pub fn pack_icon(root: &Path) -> PathBuf {
        root.join("pack_icon.png")
    }

    /// The source directory containing subdirectories for the
    /// singular packs.
    pub fn source(root: &Path) -> PathBuf {
        root.join("src")
    }

    /// The path to the source of the pack.
    pub fn source_of_pack(root: &Path, pack: Pack) -> PathBuf {
        match pack {
            Pack::Behavior => bp(root),
            Pack::Resource => rp(root),
            Pack::Skin => sp(root),
            Pack::WorldTemplate => wt(root),
        }
    }

    /// The path to the behavior pack source.
    pub fn bp(root: &Path) -> PathBuf {
        source(root).join("BP")
    }

    /// The path to the resource pack source.
    pub fn rp(root: &Path) -> PathBuf {
        source(root).join("RP")
    }

    /// The path to the skin pack source.
    pub fn sp(root: &Path) -> PathBuf {
        source(root).join("SP")
    }

    /// The path to the world template source.
    pub fn wt(root: &Path) -> PathBuf {
        source(root).join("WT")
    }

    /// The path to internal files of a project.
    pub fn internal(root: &Path) -> PathBuf {
        root.join(".allay")
    }

    /// The internal database of a project.
    pub fn db(root: &Path) -> PathBuf {
        internal(root).join("db")
    }

    /// The path to the file containing the project ID.
    pub fn project_id(root: &Path) -> PathBuf {
        db(root).join("project_id.uuid")
    }

    /// The path to the file containing the time when the project was last build for
    /// [Profile].
    pub fn last_build_for_profile(root: &Path, profile: Profile) -> PathBuf {
        match profile {
            Profile::Debug => last_build_for_debug(root),
            Profile::Release => last_build_for_release(root)
        }
    }

    /// The path to the file containing the time when the project was last built for
    /// [`Profile::Debug`].
    pub fn last_build_for_debug(root: &Path,) -> PathBuf {
        db(root).join("debug_last_build.stamp")
    }

    /// The path to the file containing the time when the project was last built for
    /// [`Profile::Release`].
    pub fn last_build_for_release(root: &Path) -> PathBuf {
        db(root).join("release_last_build.stamp")
    }

    /// The path to logs of a project.
    pub fn logs(root: &Path) -> PathBuf {
        internal(root).join("logs")
    }

    /// The path to the build directory of a project.
    pub fn build(root: &Path) -> PathBuf {
        internal(root).join("build")
    }

    /// The path to the build num file for profile [`Profile`].
    pub fn build_num_for_profile(root: &Path, profile: Profile) -> PathBuf {
        match profile {
            Profile::Debug => build_num_for_debug(root),
            Profile::Release => build_num_for_release(root),
        }
    }

    /// The path to the build num file for [`Profile::Debug`].
    pub fn build_num_for_debug(root: &Path) -> PathBuf {
        db(root).join("debug_build_num")
    }

    /// The path to the build num file for [`Profile::Release`].
    pub fn build_num_for_release(root: &Path) -> PathBuf {
        db(root).join("release_build_num")
    }

    /// The path to the [`Profile`]-specific build directory of a project.
    pub fn build_for_profile(root: &Path, profile: Profile) -> PathBuf {
        match profile {
            Profile::Debug => build_debug(root),
            Profile::Release => build_release(root),
        }
    }

    /// The path to the build directory of a project when building in [`Profile::Debug`] mode.
    pub fn build_debug(root: &Path) -> PathBuf {
        build(root).join("debug")
    }

    /// The path to the build directory of a project when building in [`Profile::Release`] mode.
    pub fn build_release(root: &Path) -> PathBuf {
        build(root).join("release")
    }

    /// The path to the directory where the source is copied to,
    /// to be modified by plugins for the specified [`Profile`].
    pub fn prebuild_for_profile(root: &Path, profile: Profile) -> PathBuf {
        match profile {
            Profile::Debug => prebuild_debug(root),
            Profile::Release => prebuild_release(root),
        }
    }

    /// The path to the directory where the source is copied to,
    /// to be modified by plugins when building in [`Profile::Debug`] mode.
    pub fn prebuild_debug(root: &Path) -> PathBuf {
        build_debug(root).join("prebuild")
    }

    /// The path to the directory where the source is copied to,
    /// to be modified by plugins when building in [`Profile::Release`] mode.
    pub fn prebuild_release(root: &Path) -> PathBuf {
        build_release(root).join("prebuild")
    }

    /// The path to the prebuild of the pack when building in [`Profile::Debug`]
    /// mode.
    pub fn prebuild_of_pack_debug(root: &Path, pack: Pack) -> PathBuf {
        match pack {
            Pack::Behavior => prebuild_bp_debug(root),
            Pack::Resource => prebuild_rp_debug(root),
            Pack::Skin => prebuild_sp_debug(root),
            Pack::WorldTemplate => prebuild_wt_debug(root),
        }
    }

    /// The path to the prebuild of the pack when building in [`Profile::Release`]
    /// mode.
    pub fn prebuild_of_pack_release(root: &Path, pack: Pack) -> PathBuf {
        match pack {
            Pack::Behavior => prebuild_bp_release(root),
            Pack::Resource => prebuild_rp_release(root),
            Pack::Skin => prebuild_sp_release(root),
            Pack::WorldTemplate => prebuild_wt_release(root),
        }
    }

    /// The path to the prebuild of the pack when building in the specified [`Profile`].
    pub fn prebuild_of_pack_for_profile(root: &Path, pack: Pack, profile: Profile) -> PathBuf {
        match profile {
            Profile::Debug => prebuild_of_pack_debug(root, pack),
            Profile::Release => prebuild_of_pack_release(root, pack),
        }
    }

    /// The path to the BP prebuild when building in [`Profile::Debug`] mode.
    pub fn prebuild_bp_debug(root: &Path) -> PathBuf {
        prebuild_debug(root).join("BP")
    }

    /// The path to the BP prebuild when building in [`Profile::Release`] mode.
    pub fn prebuild_bp_release(root: &Path) -> PathBuf {
        prebuild_release(root).join("BP")
    }

    /// The path to the RP prebuild when building in [`Profile::Debug`] mode.
    pub fn prebuild_rp_debug(root: &Path) -> PathBuf {
        prebuild_debug(root).join("RP")
    }

    /// The path to the RP prebuild when building in [`Profile::Release`] mode.
    pub fn prebuild_rp_release(root: &Path) -> PathBuf {
        prebuild_release(root).join("RP")
    }

    /// The path the the SP prebuild when building in [`Profile::Debug`] mode.
    pub fn prebuild_sp_debug(root: &Path) -> PathBuf {
        prebuild_debug(root).join("SP")
    }

    /// The path the the SP prebuild when building in [`Profile::Release`] mode.
    pub fn prebuild_sp_release(root: &Path) -> PathBuf {
        prebuild_release(root).join("SP")
    }

    /// The path to the WT prebuild when building in [`Profile::Debug`] mode.
    pub fn prebuild_wt_debug(root: &Path) -> PathBuf {
        prebuild_debug(root).join("WT")
    }

    /// The path to the WT prebuild when building in [`Profile::Release`] mode.
    pub fn prebuild_wt_release(root: &Path) -> PathBuf {
        prebuild_release(root).join("WT")
    }

    /// The path to the final build file for [`Profile`] builds.
    pub fn build_file_for_profile(root: &Path, profile: Profile) -> PathBuf {
        match profile {
            Profile::Debug => build_file_debug(root),
            Profile::Release => build_file_release(root),
        }
    }

    /// The path to the final build file for [`Profile::Debug`] builds.
    pub fn build_file_debug(root: &Path) -> PathBuf {
        build_debug(root).join("build-debug.mcaddon")
    }

    /// The path to the final build file for [`Profile::Release`] builds.
    pub fn build_file_release(root: &Path) -> PathBuf {
        build_release(root).join("build-release.mcaddon")
    }
}

/// Non-project-specific paths.
pub mod global {
    use std::path::PathBuf;

    /// The directory where Allay related files are stored.
    pub fn data() -> PathBuf {
        #[allow(clippy::expect_used)]
        dirs::data_local_dir()
            .expect("unsupported OS")
            .join("allay")
    }

    /// The path to the templates directory.
    pub fn templates() -> PathBuf {
        data().join("templates")
    }

    /// The path to logs generated by Allay.
    pub fn logs() -> PathBuf {
        data().join("logs")
    }

    /// The path to the database directory.
    pub fn db() -> PathBuf {
        data().join("db")
    }

    /// The JSON schema for the Allay configuration file (`allay.toml`) compatible with the
    /// currently used version of Allay.
    pub fn schema_version_specific() -> PathBuf {
        db().join(format!("config-{}.schema.json", crate::VERSION))
    }

    /// The JSON schema for the Allay configuration file (`allay.toml`) compatible with the
    /// most recently used version of Allay.
    pub fn schema() -> PathBuf {
        db().join("config.schema.json")
    }

    /// The path to the local installation of the manual.
    pub fn manual() -> PathBuf {
        data().join("manual")
    }
}
