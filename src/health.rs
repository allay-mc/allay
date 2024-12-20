//! Utilities for validating and fixing the project structure and data.

use crate::uuid::Uuids;
use crate::{paths, uuid};
use std::fs;
use std::path::PathBuf;

pub struct Health {
    /// The root directory of the project.
    pub root: PathBuf,

    /// Whether to fix missing or corrupted files.
    pub fix: bool,
}

impl Health {
    pub fn check_all(&self) -> bool {
        for success in [
            self.check_internal(),
            self.check_uuids(),
            self.check_uuids_presence(),
        ] {
            if !success {
                return false;
            }
        }
        true
    }

    pub fn check_all_except_uuids(&self) -> bool {
        for success in [self.check_internal(), self.check_uuids()] {
            if !success {
                return false;
            }
        }
        true
    }

    pub fn check_internal(&self) -> bool {
        let internal = self.root.join(paths::internal());
        if internal.exists() {
            true
        } else if self.fix {
            log::info!("Fix missing internal directory");
            fs::create_dir(internal).expect("failed to fix issue");
            true
        } else {
            log::error!("Internal directory is missing; try `allay health --fix`");
            false
        }
    }

    pub fn check_uuids(&self) -> bool {
        let uuids = self.root.join(paths::uuids());
        let valid_file_format: bool = (|| {
            toml::from_str::<toml::Value>(match fs::read_to_string(&uuids) {
                Ok(ref data) => data,
                Err(e) => {
                    log::error!("Failed to read UUID file: {}", e);
                    return false;
                }
            })
            .is_ok()
        })();
        if uuids.exists() && valid_file_format {
            true
        } else if self.fix {
            log::info!("Fix missing UUIDs");
            let data = uuid::Uuids::default();
            fs::write(uuids, data.to_string()).expect("failed to fill UUIDs");
            true
        } else {
            log::error!("UUIDs are missing; try `allay health --fix`");
            false
        }
    }

    pub fn check_uuids_presence(&self) -> bool {
        let uuids = self.root.join(paths::uuids());
        let content = match fs::read_to_string(&uuids) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Failed to read UUID file: {}", e);
                if self.fix {
                    let data = uuid::Uuids::default();
                    let content = data.to_string();
                    fs::write(&uuids, &content).expect("failed to fill UUIDs");
                    content
                } else {
                    return false;
                }
            }
        };
        let data: Result<Uuids, _> = toml::from_str(&content);
        match data {
            Err(e) => {
                log::error!("Invalid TOML: {}", e);
                false
            }
            Ok(mut data) => {
                let mut modified = false;
                let mut ok = true;

                if has_content(&self.root.join(paths::src_bp())) {
                    if data.bp.header.is_none() {
                        if self.fix {
                            log::info!("Add missing BP header UUID");
                            data.bp.update_header(None);
                            modified = true;
                        } else {
                            log::error!("BP header UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                    if data.bp.module.is_none() {
                        if self.fix {
                            log::info!("Add missing BP module UUID");
                            data.bp.update_module(None);
                            modified = true;
                        } else {
                            log::error!("BP module UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                }
                if has_content(&self.root.join(paths::src_rp())) {
                    if data.rp.header.is_none() {
                        if self.fix {
                            log::info!("Add missing RP header UUID");
                            data.rp.update_header(None);
                            modified = true;
                        } else {
                            log::error!("RP header UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                    if data.rp.module.is_none() {
                        if self.fix {
                            log::info!("Add missing RP module UUID");
                            data.rp.update_module(None);
                            modified = true;
                        } else {
                            log::error!("RP module UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                }
                if has_content(&self.root.join(paths::src_sp())) {
                    if data.sp.header.is_none() {
                        if self.fix {
                            log::info!("Add missing SP header UUID");
                            data.sp.update_header(None);
                            modified = true;
                        } else {
                            log::error!("BP header UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                    if data.sp.module.is_none() {
                        if self.fix {
                            log::info!("Add missing SP module UUID");
                            data.sp.update_module(None);
                            modified = true;
                        } else {
                            log::error!("SP module UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                }
                if has_content(&self.root.join(paths::src_wt())) {
                    if data.wt.header.is_none() {
                        if self.fix {
                            log::info!("Add missing WT header UUID");
                            data.wt.update_header(None);
                            modified = true;
                        } else {
                            log::error!("WT header UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                    if data.wt.module.is_none() {
                        if self.fix {
                            log::info!("Add missing WT module UUID");
                            data.wt.update_module(None);
                            modified = true;
                        } else {
                            log::error!("WT module UUID is missing; try `allay health --fix`");
                            ok = false;
                        }
                    }
                }

                if modified {
                    // write back to UUID file
                    if let Err(e) = fs::write(&uuids, data.to_string()) {
                        log::error!("Error while writing to UUID file: {}", e);
                        ok = false;
                    };
                };

                ok
            }
        }
    }
}

/// Returns `true` when `dir` contains files or directories.
pub fn has_content(dir: &PathBuf) -> bool {
    dir.read_dir()
        .map(|entries| entries.count() > 0)
        .unwrap_or_default()
}
