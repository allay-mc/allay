pub(crate) mod localization;
pub(crate) mod manifest;
pub(crate) mod uuidgen;

use std::fs;

use anyhow::Context;
use fs_extra::dir::{copy, CopyOptions};

use crate::addon::AddonType;
use crate::build::uuidgen::update_uuids;
use crate::environment::Environment;
use crate::paths;
use crate::scripts;

/// Text added at the end of the file stem to highlight a development release.
pub(crate) const DEVELOPMENT_SUFFIX: &'static str = "_dev";

/// Text added at the end of the file stem to highlight a production release.
pub(crate) const PRODUCTION_SUFFIX: &'static str = "";

pub(crate) fn build(env: &mut Environment) -> anyhow::Result<()> {
    let options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64_000,
        copy_inside: false,
        content_only: true,
        depth: 0,
    };

    // TODO: move log messages to the corresponding functions instead of here
    //       in case of `if`s

    update_uuids(env).with_context(|| "cannot save UUIDs")?;

    // copy to prebuild and generate manifest
    for pack in AddonType::all() {
        if !pack.exists() {
            log::info!("skipping {} because it has no content", pack);
            continue;
        }
        log::debug!("copying {} source into prebuild", pack);
        let dest = paths::root().join(pack.path_prebuild());
        copy(paths::root().join(pack.path_src()), dest, &options)
            .with_context(|| "cannot copy to prebuild")?;
        log::info!("generating manifest for {}", pack);
        let mf = serde_json::to_string(&match pack {
            AddonType::BehaviorPack => manifest::behavior_pack(env),
            AddonType::ResourcePack => manifest::resource_pack(env),
            AddonType::SkinPack => manifest::skin_pack(env),
            AddonType::WorldTemplate => manifest::world_template(env),
        })
        .with_context(|| "cannot parse manifest")?;
        let p = paths::root()
            .join(pack.path_prebuild())
            .join("manifest.json");
        fs::write(p, mf).with_context(|| "cannot write manifest")?;
    }

    // add pack icons
    let icon = paths::root().join(paths::pack_icon());
    if icon.exists() {
        let icon_bp = paths::root()
            .join(paths::prebuild_bp())
            .join("pack_icon.png");
        if icon_bp.exists() {
            log::info!("Copying project icon into BP");
            if fs::copy(&icon, icon_bp).is_err() {
                log::error!("Could not copy project icon to BP")
            };
        }

        let icon_rp = paths::root()
            .join(paths::prebuild_rp())
            .join("pack_icon.png");
        if icon_rp.exists() {
            log::info!("Copying project icon into RP");
            if fs::copy(&icon, icon_rp).is_err() {
                log::error!("Could not copy project icon to RP")
            };
        }

        let icon_sp = paths::root()
            .join(paths::prebuild_sp())
            .join("pack_icon.png");
        if icon_sp.exists() {
            log::info!("Copying project icon into SP");
            if fs::copy(&icon, icon_sp).is_err() {
                log::error!("Could not copy project icon to SP")
            };
        }
    }

    log::info!("appending language files");
    if let anyhow::Result::Err(e) = localization::append_language_files(env) {
        log::error!("failed to append language files: {}", e);
    }

    log::info!("running pre scripts");
    if let anyhow::Result::Err(e) = scripts::run_pre_scripts(env) {
        log::error!("failed to run pre scripts: {}", e);
    }

    log::info!("copying into build directory");
    let source = paths::root().join(paths::prebuild());
    if source.is_dir() {
        fs::remove_dir_all(paths::root().join(paths::build_bp())).unwrap_or(());
        fs::remove_dir_all(paths::root().join(paths::build_rp())).unwrap_or(());
        fs::remove_dir_all(paths::root().join(paths::build_sp())).unwrap_or(());
        fs::remove_dir_all(paths::root().join(paths::build_wt())).unwrap_or(());

        copy(source, paths::root().join(paths::build()), &options)
            .with_context(|| "cannot copy to build")?;

        for pack in AddonType::all() {
            let dest = paths::root().join(paths::build()).join(format!(
                "{}_{}_v{}{}",
                pack.short_name(),
                env.config
                    .as_ref()
                    .unwrap()
                    .project
                    .name
                    .get(&env.config.as_ref().unwrap().localization.primary_language)
                    .with_context(|| "missing localized string for primary language")?
                    .replace(' ', "_"),
                env.config.as_ref().unwrap().project.version,
                match env.development {
                    Some(true) => DEVELOPMENT_SUFFIX,
                    Some(false) => PRODUCTION_SUFFIX,
                    None => unreachable!(),
                }
            ));
            if dest.exists() {
                log::debug!("removing previous build");
                fs::remove_dir_all(dest.clone())
                    .with_context(|| "failed to remove previous build")?;
            }
            if pack.exists() {
                log::debug!("copying {} build", pack);
                fs::rename(paths::root().join(pack.path_build()), dest.clone())
                    .with_context(|| "cannot rename")?;

                #[cfg(target_family = "unix")]
                let res = std::os::unix::fs::symlink(dest.clone(), pack.path_build());
                #[cfg(target_family = "windows")]
                let res = std::os::windows::fs::symlink_dir(dest.clone(), pack.path_build());

                if let std::io::Result::Err(e) = res {
                    log::error!("failed to symlink {}: {}", pack, e);
                }
            }
        }
    }

    // TODO: if running in release mode, zip the build into mcpack depending on config

    log::info!("running post scripts");
    if let anyhow::Result::Err(e) = scripts::run_post_scripts(env) {
        log::error!("failed to run post scripts: {}", e);
    };

    log::debug!("clear prebuild");
    fs::remove_dir_all(paths::prebuild()).with_context(|| "failed to remove prebuild directory")?;
    fs::create_dir(paths::prebuild()).with_context(|| "failed to create prebuild")?;

    Ok(())
}
