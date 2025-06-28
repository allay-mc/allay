//! Lock file utilities for Allay.
//!
//! The lock file stores UUIDs used for the add-ons.

// FIXME: user should be warned if missing necessary module entries in config file
// TODO: add comment to top of lock to tell the user it should not be modified

use std::{collections::HashMap, str::FromStr};

use crate::{
    config::{BehaviorPackModuleKind, ResourcePackModuleKind},
    manifest,
    project::health,
    Pack, Project,
};
use serde::{Deserialize, Serialize};

/// The lock data that stores UUIDs.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Lock {
    /// Information related to the version of Allay beeing used.
    pub allay: Allay,

    /// The UUIDs used within a project.
    pub uuid_table: UuidTable,
}

impl Lock {
    /// Creates a new bare lock.
    pub fn new() -> Self {
        Self {
            allay: Allay::default(),
            uuid_table: UuidTable::new(),
        }
    }
}

impl Default for Lock {
    fn default() -> Self {
        Self::new()
    }
}

/// Information related to the version of Allay beeing used.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Allay {
    /// The version of Allay.
    pub version: semver::Version,
}

impl Default for Allay {
    fn default() -> Self {
        Self {
            version: semver::Version::from_str(crate::VERSION).unwrap(),
        }
    }
}

/// The UUIDs used within a project.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UuidTable {
    /// UUID data for the behavior pack.
    pub bp: Option<Uuids>,

    /// UUID data for the resource pack.
    pub rp: Option<Uuids>,

    /// UUID data for the skin pack.
    pub sp: Option<Uuids>,

    /// UUID data for the world template.
    pub wt: Option<Uuids>,
}

impl UuidTable {
    /// Creates a new UUID table containing no data.
    pub fn new() -> Self {
        Self {
            bp: None,
            rp: None,
            sp: None,
            wt: None,
        }
    }

    /// Retrieves the UUID data for a given [`Pack`].
    pub fn get(&self, pack: &Pack) -> Option<&Uuids> {
        match pack {
            Pack::Behavior => self.bp.as_ref(),
            Pack::Resource => self.rp.as_ref(),
            Pack::Skin => self.sp.as_ref(),
            Pack::WorldTemplate => self.wt.as_ref(),
        }
    }
}

impl Default for UuidTable {
    fn default() -> Self {
        Self::new()
    }
}

/// The UUID data for a pack.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Uuids {
    /// The header UUID.
    pub header: UuidEntry,

    /// The modules of the pack.
    pub modules: HashMap<manifest::ModuleKind, UuidEntry>,
}

/// The UUID and version that can be referenced by external packs for dependencies.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UuidEntry {
    /// The UUID itself.
    pub uuid: uuid::Uuid,

    /// The version of the header/module.
    pub version: manifest::Version,
}

/// Loads a lock from TOML.
pub fn load(document: &str) -> Result<Lock, toml::de::Error> {
    toml::from_str(document)
}

/// Writes a lock to a TOML string.
pub fn write(lock: &Lock) -> String {
    // TODO: toml crate supports toml_edit, so use a formatter for that
    toml::ser::to_string_pretty(lock).unwrap()
}

#[must_use]
fn update_existing_uuids(
    uuids: Option<&Uuids>,
    project_version: manifest::Version,
    pack_modules: Vec<(manifest::ModuleKind, manifest::Version)>,
) -> Uuids {
    if let Some(uuid_table) = uuids {
        let header = uuid_table.header.clone();
        let mut modules: HashMap<manifest::ModuleKind, UuidEntry> = HashMap::new();
        for (module_kind, module_version) in pack_modules {
            match uuid_table.modules.get(&module_kind) {
                Some(uuid_entry) => {
                    modules.insert(module_kind, uuid_entry.clone());
                }
                None => {
                    let uuid_entry = UuidEntry {
                        uuid: uuid::Uuid::new_v4(),
                        version: module_version,
                    };
                    modules.insert(module_kind, uuid_entry);
                }
            }
        }
        Uuids { header, modules }
    } else {
        let header = UuidEntry {
            uuid: uuid::Uuid::new_v4(),
            version: project_version,
        };
        let mut modules: HashMap<manifest::ModuleKind, UuidEntry> = HashMap::new();
        for (module_kind, module_version) in pack_modules {
            let uuid_entry = UuidEntry {
                uuid: uuid::Uuid::new_v4(),
                version: module_version,
            };
            modules.insert(module_kind, uuid_entry);
        }
        Uuids { header, modules }
    }
}

/// Updates a project's lock.
///
/// This function returns a new lock which should be used as the
/// new lock for the project. The old lock of the project
/// persists.
pub fn update(project: &Project) -> Lock {
    // TODO: backup removed UUIDs
    let mut lock = project.lock.clone().unwrap_or_default();

    if health::source_has_content(project, &Pack::Behavior) {
        lock.uuid_table.bp = Some(update_existing_uuids(
            lock.uuid_table.bp.as_ref(),
            project.config.project.version.clone().into(),
            project
                .config
                .bp
                .modules
                .iter()
                .map(|module| {
                    (
                        match module.kind {
                            BehaviorPackModuleKind::Data(_) => manifest::ModuleKind::Data,
                            BehaviorPackModuleKind::Script(_) => manifest::ModuleKind::Script,
                        },
                        module.version.clone().into(),
                    )
                })
                .collect(),
        ));
    } else {
        lock.uuid_table.bp = None;
    }

    if health::source_has_content(project, &Pack::Resource) {
        lock.uuid_table.rp = Some(update_existing_uuids(
            lock.uuid_table.rp.as_ref(),
            project.config.project.version.clone().into(),
            project
                .config
                .rp
                .modules
                .iter()
                .map(|module| {
                    (
                        match module.kind {
                            ResourcePackModuleKind::Resources => manifest::ModuleKind::Resources,
                        },
                        module.version.clone().into(),
                    )
                })
                .collect(),
        ));
    } else {
        lock.uuid_table.rp = None;
    }

    if health::source_has_content(project, &Pack::Skin) {
        lock.uuid_table.sp = Some(update_existing_uuids(
            lock.uuid_table.sp.as_ref(),
            project.config.project.version.clone().into(),
            Vec::new(),
        ));
    } else {
        lock.uuid_table.sp = None;
    }

    if health::source_has_content(project, &Pack::WorldTemplate) {
        lock.uuid_table.wt = Some(update_existing_uuids(
            lock.uuid_table.wt.as_ref(),
            project.config.project.version.clone().into(),
            Vec::new(),
        ));
    } else {
        lock.uuid_table.wt = None;
    }

    lock
}
