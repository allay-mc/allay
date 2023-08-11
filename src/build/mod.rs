pub(crate) mod localization;
pub(crate) mod manifest;
pub(crate) mod uuidgen;

use std::fs;

use fs_extra::dir::{copy, CopyOptions};
use uuid::Uuid;

use crate::environment::Environment;
use crate::paths;
use crate::scripts;
use crate::utils::{empty_dir, has_bp, has_rp, has_sp, has_wt};

fn update_uuids(env: &mut Environment) -> std::io::Result<()> {
    if has_bp() {
        if uuidgen::bp_header(&env.uuids).is_none() {
            log::debug!("update BP header UUID");
            uuidgen::update_bp_header(&mut env.uuids, Uuid::new_v4().to_string());
        }

        if uuidgen::bp_module(&env.uuids).is_none() {
            log::debug!("update BP module UUID");
            uuidgen::update_bp_module(&mut env.uuids, Uuid::new_v4().to_string());
        }
    }

    if has_rp() {
        if uuidgen::rp_header(&env.uuids).is_none() {
            log::debug!("update RP header UUID");
            uuidgen::update_rp_header(&mut env.uuids, Uuid::new_v4().to_string());
        }

        if uuidgen::rp_module(&env.uuids).is_none() {
            log::debug!("update RP module UUID");
            uuidgen::update_rp_module(&mut env.uuids, Uuid::new_v4().to_string());
        }
    }

    if has_sp() {
        if uuidgen::sp_header(&env.uuids).is_none() {
            log::debug!("update SP header UUID");
            uuidgen::update_sp_header(&mut env.uuids, Uuid::new_v4().to_string());
        }

        if uuidgen::sp_module(&env.uuids).is_none() {
            log::debug!("update SP module UUID");
            uuidgen::update_sp_module(&mut env.uuids, Uuid::new_v4().to_string());
        }
    }

    if has_wt() {
        if uuidgen::wt_header(&env.uuids).is_none() {
            log::debug!("update WT header UUID");
            uuidgen::update_wt_header(&mut env.uuids, Uuid::new_v4().to_string());
        }

        if uuidgen::wt_module(&env.uuids).is_none() {
            log::debug!("update WT module UUID");
            uuidgen::update_wt_module(&mut env.uuids, Uuid::new_v4().to_string());
        }
    }

    uuidgen::save_uuids(&env.uuids)
}

pub(crate) fn build(env: &mut Environment) {
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

    update_uuids(env).expect("cannot save UUIDs");

    if has_bp() {
        log::debug!("copying src/BP to prebuild/BP");
        let dest = paths::prebuild_bp();
        copy(paths::src_bp(), dest, &options).expect("cannot copy to prebuild");

        log::info!("creating BP manifest");
        let m =
            serde_json::to_string(&manifest::behavior_pack(env)).expect("cannot parse manifest");
        let p = paths::prebuild_bp().join("manifest.json");
        fs::write(p, m).expect("cannot write manifest");
    } else {
        log::debug!("skipping src/BP because it has no content");
    };

    if has_rp() {
        log::debug!("copying src/RP to prebuild/RP");
        let dest = paths::prebuild_rp();
        copy(paths::src_rp(), dest, &options).expect("copy to prebuild");

        log::info!("creating RP manifest");
        let m =
            serde_json::to_string(&manifest::resource_pack(env)).expect("cannot parse manifest");
        let p = paths::prebuild_rp().join("manifest.json");
        fs::write(p, m).expect("cannot write manifest");
    } else {
        log::debug!("skipping copying src/RP because it has no content");
    };

    if has_sp() {
        log::debug!("copying src/SP to prebuild/SP");
        let dest = paths::prebuild_sp();
        copy(paths::src_sp(), dest, &options).expect("cannot copy to prebuild");

        log::info!("creating SP manifest");
        let m = serde_json::to_string(&manifest::skin_pack(env)).expect("cannot parse manifest");
        let p = paths::prebuild_sp().join("manifest.json");
        fs::write(p, m).expect("cannot write manifest");
    } else {
        log::debug!("skipping copying src/SP because it has no content");
    };

    if has_wt() {
        log::debug!("copying src/WT to prebuild/WT");
        let dest = paths::prebuild_wt();
        copy(paths::src_wt(), dest, &options).expect("cannot copy to prebuild");

        log::info!("creating WT manifest");
        let m =
            serde_json::to_string(&manifest::world_template(env)).expect("cannot parse manifest");
        let p = paths::prebuild_wt().join("manifest.json");
        fs::write(p, m).expect("cannot write manifest");
    } else {
        log::debug!("skipping copying src/WT because it has no content");
    };

    // append localization
    log::info!("appending language files");
    localization::append_language_files(env);

    log::info!("running pre scripts");
    scripts::run_pre_scripts(env).expect("cannot run pre scripts");

    log::info!("copying into build directory");
    let source = paths::prebuild();
    if source.is_dir() {
        fs::remove_dir_all(paths::build_bp()).unwrap_or(());
        fs::remove_dir_all(paths::build_rp()).unwrap_or(());
        fs::remove_dir_all(paths::build_sp()).unwrap_or(());
        fs::remove_dir_all(paths::build_wt()).unwrap_or(());

        copy(source, paths::build(), &options).expect("cannot copy to build");

        let dest = paths::build().join(format!(
            "BP_{}_v{}{}",
            env.config
                .project
                .name
                .get(&env.config.localization.primary_language)
                .expect("missing localized string for primary language")
                .replace(' ', "_"),
            env.config.project.version,
            if env.development { "_dev" } else { "" },
        ));
        if dest.exists() {
            log::debug!("removing previous build");
            fs::remove_dir_all(dest.clone()).expect("cannot remove previous build");
        };
        if has_bp() {
            log::debug!("copying bp build");
            fs::rename(paths::build_bp(), dest.clone()).expect("cannot rename");

            #[cfg(target_family = "unix")]
            std::os::unix::fs::symlink(dest.clone(), paths::build_bp())
                .expect("cannot symlink bp build");
            #[cfg(target_family = "windows")]
            std::os::windows::fs::symlink_dir(dest.clone(), paths::build_bp())
                .expect("cannot symlink bp build");
        };

        let dest = paths::build().join(format!(
            "RP_{}_v{}{}",
            env.config
                .project
                .name
                .get(&env.config.localization.primary_language)
                .expect("missing localized string for primary language")
                .replace(' ', "_"),
            env.config.project.version,
            if env.development { "_dev" } else { "" },
        ));
        if dest.exists() {
            log::debug!("removing previous build");
            fs::remove_dir_all(dest.clone()).expect("cannot remove previous build");
        };
        if has_rp() {
            log::debug!("copying rp build");
            fs::rename(paths::build_rp(), dest.clone()).expect("cannot rename");

            #[cfg(target_family = "unix")]
            std::os::unix::fs::symlink(dest.clone(), paths::build_rp())
                .expect("cannot symlink bp build");
            #[cfg(target_family = "windows")]
            std::os::windows::fs::symlink_dir(dest.clone(), paths::build_rp())
                .expect("cannot symlink bp build");
        };

        let dest = paths::build().join(format!(
            "SP_{}_v{}{}",
            env.config
                .project
                .name
                .get(&env.config.localization.primary_language)
                .expect("missing localized string for primary language")
                .replace(' ', "_"),
            env.config.project.version,
            if env.development { "_dev" } else { "" },
        ));
        if dest.exists() {
            log::debug!("removing previous build");
            fs::remove_dir_all(dest.clone()).expect("cannot remove previous build");
        };
        if has_sp() {
            log::debug!("copying sp build");
            fs::rename(paths::build_sp(), dest.clone()).expect("cannot rename");

            #[cfg(target_family = "unix")]
            std::os::unix::fs::symlink(dest.clone(), paths::build_sp())
                .expect("cannot symlink bp build");
            #[cfg(target_family = "windows")]
            std::os::windows::fs::symlink_dir(dest.clone(), paths::build_sp())
                .expect("cannot symlink bp build");
        };

        let dest = paths::build().join(format!(
            "WT_{}_v{}{}",
            env.config
                .project
                .name
                .get(&env.config.localization.primary_language)
                .expect("missing localized string for primary language")
                .replace(' ', "_"),
            env.config.project.version,
            if env.development { "_dev" } else { "" },
        ));
        if dest.exists() {
            log::debug!("removing previous build");
            fs::remove_dir_all(dest.clone()).expect("cannot remove previous build");
        };
        if has_wt() {
            log::debug!("copying wt build");
            fs::rename(paths::build_wt(), dest.clone()).expect("cannot rename");

            #[cfg(target_family = "unix")]
            std::os::unix::fs::symlink(dest.clone(), paths::build_wt())
                .expect("cannot symlink bp build");
            #[cfg(target_family = "windows")]
            std::os::windows::fs::symlink_dir(dest.clone(), paths::build_wt())
                .expect("cannot symlink bp build");
        };
    }

    // TODO: if running in release mode, zip the build into mcpack depending on config

    log::info!("running post scripts");
    scripts::run_post_scripts(env).expect("cannot run post scripts");

    log::debug!("clear prebuild");
    fs::remove_dir_all(paths::prebuild()).expect("cannot remove prebuild directory");
    fs::create_dir(paths::prebuild()).expect("cannot create prebuild");
}
