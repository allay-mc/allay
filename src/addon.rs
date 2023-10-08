use std::{fmt, path::PathBuf};

use clap::builder::PossibleValue;

use crate::{paths, utils::empty_dir};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum AddonType {
    BehaviorPack,
    ResourcePack,
    SkinPack,
    WorldTemplate,
}

impl fmt::Display for AddonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddonType::BehaviorPack => write!(f, "Behavior Pack"),
            AddonType::ResourcePack => write!(f, "Resource Pack"),
            AddonType::SkinPack => write!(f, "Skin Pack"),
            AddonType::WorldTemplate => write!(f, "World Template"),
        }
    }
}

impl clap::ValueEnum for AddonType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            AddonType::BehaviorPack,
            AddonType::ResourcePack,
            AddonType::SkinPack,
            AddonType::WorldTemplate,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            AddonType::BehaviorPack => PossibleValue::new("bp"),
            AddonType::ResourcePack => PossibleValue::new("rp"),
            AddonType::SkinPack => PossibleValue::new("sp"),
            AddonType::WorldTemplate => PossibleValue::new("wt"),
        })
    }
}

impl AddonType {
    pub(crate) fn all() -> &'static [Self] {
        &[
            AddonType::BehaviorPack,
            AddonType::ResourcePack,
            AddonType::SkinPack,
            AddonType::WorldTemplate,
        ]
    }

    pub(crate) fn short_name(&self) -> &'static str {
        match self {
            AddonType::BehaviorPack => "BP",
            AddonType::ResourcePack => "RP",
            AddonType::SkinPack => "SP",
            AddonType::WorldTemplate => "WT",
        }
    }

    pub(crate) fn long_name(&self) -> &'static str {
        match self {
            AddonType::BehaviorPack => "Behavior Pack",
            AddonType::ResourcePack => "Resource Pack",
            AddonType::SkinPack => "Skin Pack",
            AddonType::WorldTemplate => "World Template",
        }
    }

    pub(crate) fn path_src(&self) -> PathBuf {
        match self {
            AddonType::BehaviorPack => paths::src_bp(),
            AddonType::ResourcePack => paths::src_rp(),
            AddonType::SkinPack => paths::src_sp(),
            AddonType::WorldTemplate => paths::src_wt(),
        }
    }

    pub(crate) fn path_build(&self) -> PathBuf {
        match self {
            AddonType::BehaviorPack => paths::build_bp(),
            AddonType::ResourcePack => paths::build_rp(),
            AddonType::SkinPack => paths::build_sp(),
            AddonType::WorldTemplate => paths::build_wt(),
        }
    }

    pub(crate) fn path_prebuild(&self) -> PathBuf {
        match self {
            AddonType::BehaviorPack => paths::prebuild_bp(),
            AddonType::ResourcePack => paths::prebuild_rp(),
            AddonType::SkinPack => paths::prebuild_sp(),
            AddonType::WorldTemplate => paths::prebuild_wt(),
        }
    }

    pub(crate) fn exists(&self) -> bool {
        !empty_dir(&paths::root().join(self.path_src())).unwrap_or(true)
    }
}
