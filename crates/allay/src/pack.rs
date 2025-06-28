//! Utilities related to packs (behavior pack, resource pack, skin pack, world template).

use std::{fmt, path::{Path, PathBuf}};

use serde::Deserialize;

/// Returns all kinds of packs.
pub const fn packs() -> [Pack; 4] {
    [
        Pack::Behavior,
        Pack::Resource,
        Pack::Skin,
        Pack::WorldTemplate,
    ]
}

/// All kind of packs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Pack {
    #[allow(missing_docs)]
    Behavior,

    #[allow(missing_docs)]
    Resource,

    #[allow(missing_docs)]
    Skin,

    #[allow(missing_docs)]
    WorldTemplate,
}

impl Pack {
    /// Returns a string representation.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Behavior => "Behavior Pack",
            Self::Resource => "Resource Pack",
            Self::Skin => "Skin Pack",
            Self::WorldTemplate => "World Template",
        }
    }

    /// Returns the ID of the pack.
    pub const fn id(&self) -> &'static str {
        match self {
            Self::Behavior => "bp",
            Self::Resource => "rp",
            Self::Skin => "sp",
            Self::WorldTemplate => "wt",
        }
    }

    /// Returns the source path of the pack.
    pub fn path_source(&self, root: &Path) -> PathBuf {
        match self {
            Self::Behavior => crate::paths::project::bp(root),
            Self::Resource => crate::paths::project::rp(root),
            Self::Skin => crate::paths::project::sp(root),
            Self::WorldTemplate => crate::paths::project::wt(root),
        }
    }
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
