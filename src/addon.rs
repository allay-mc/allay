use std::fmt;

use clap::builder::PossibleValue;

#[derive(Clone, Copy)]
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
