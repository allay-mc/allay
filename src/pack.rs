use crate::paths;
use clap::builder::PossibleValue;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pack {
    Behavior,
    Resource,
    Skin,
    WorldTemplate,
}

impl clap::ValueEnum for Pack {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Behavior,
            Self::Resource,
            Self::Skin,
            Self::WorldTemplate,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Behavior => PossibleValue::new("BP"),
            Self::Resource => PossibleValue::new("RP"),
            Self::Skin => PossibleValue::new("SP"),
            Self::WorldTemplate => PossibleValue::new("WT"),
        })
    }
}

impl Pack {
    pub const VALUES: [Self; 4] = [
        Self::Behavior,
        Self::Resource,
        Self::Skin,
        Self::WorldTemplate,
    ];

    /// Returns the bundle file extension for a pack like `mcpack` for behavior packs.
    ///
    /// # References
    ///
    /// - <https://learn.microsoft.com/en-us/minecraft/creator/documents/minecraftfileextensions?view=minecraft-bedrock-stable#mcaddon>
    pub fn bundle_file_extension(&self) -> &'static str {
        match self {
            Self::Behavior | Self::Resource | Self::Skin => "mcpack",
            Self::WorldTemplate => "mctemplate",
        }
    }

    pub fn path_src(&self) -> Option<PathBuf> {
        Some(paths::try_root()?.join(match self {
            Self::Behavior => paths::src_bp(),
            Self::Resource => paths::src_rp(),
            Self::Skin => paths::src_sp(),
            Self::WorldTemplate => paths::src_wt(),
        }))
    }

    /// Returns `true` when the source directory for the pack exists and has at least one entry.
    pub fn exists(&self) -> bool {
        self.path_src()
            .is_some_and(|p| p.read_dir().is_ok_and(|it| it.count().gt(&0)))
    }

    /// Returns the short name for the pack (e.g. "BP" for [`Pack::Behavior`]).
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Behavior => "BP",
            Self::Resource => "RP",
            Self::Skin => "SP",
            Self::WorldTemplate => "WT",
        }
    }
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Behavior => "Behavior Pack",
                Self::Resource => "Resource Pack",
                Self::Skin => "Skin Pack",
                Self::WorldTemplate => "World Template",
            }
        )
    }
}
