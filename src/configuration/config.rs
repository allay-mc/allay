use std::collections::HashMap;

use serde::Deserialize;

use super::manifest::{Capabilities, Dependency, ModuleType};

pub(crate) type Language = String;
pub(crate) type Localized<T> = HashMap<Language, T>;
pub(crate) type Version = String;

pub(crate) mod defaults {
    use super::super::localization;
    pub(crate) fn localization_groups() -> Vec<Vec<String>> {
        localization::groups()
            .iter()
            .map(|v| v.iter().map(|&s| s.into()).collect())
            .collect()
    }

    pub(crate) fn ret_true() -> bool {
        true
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
pub(crate) struct Config {
    /// Optional reference to schema.
    #[serde(rename = "$schema")]
    #[serde(default)]
    pub schema: Option<String>,

    /// Metadata of the Allay project.
    pub project: Project,

    /// Scripts run before and after the build.
    #[serde(default)]
    pub scripts: Scripts,
    // /// Metadata that may be used by external tools.
    // #[serde(borrow)]
    // pub metadata: HashMap<&'a str, toml::Value>,
    #[serde(default)]
    pub pack: Pack,

    pub localization: Localization,

    #[serde(default)]
    pub build: Build,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Project {
    /// The name of the project.
    pub name: Localized<String>,

    /// The decription of the project.
    pub description: Localized<String>,

    /// The version of the project.
    pub version: Version,

    /// The authors of the project.
    #[serde(default)]
    pub authors: Option<Vec<String>>,

    /// SPDX license identifier of the project.
    pub license: Option<String>,

    /// URL to the homepage of your project.
    pub url: Option<String>,

    /// Whether this project is still in development. This influences the build
    /// process and can be overridden with the `--release` flag.
    #[serde(default = "defaults::ret_true")] // https://github.com/serde-rs/serde/issues/368 :/
    pub development: bool,

    /// Minimum version of the game the pack is written for.
    pub min_engine_version: Version,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Localization {
    /// The preferred language each other one falls back to.
    pub primary_language: Language,

    #[serde(default = "defaults::localization_groups")]
    pub localization_groups: Vec<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Build {
    /// Always uses this type of build.
    #[serde(default)]
    pub build_type: Option<BuildType>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BuildType {
    Mcpack,
    Directory,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Scripts {
    /// Prefix each entry with this path.
    #[serde(default)]
    pub base_path: String,

    /// Scripts run before the build.
    #[serde(default)]
    pub pre: Vec<Script>,

    /// Scripts run after the build.
    #[serde(default)]
    pub post: Vec<Script>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
pub(crate) struct Script {
    /// The path to the script.
    pub run: String,

    /// The program to run the script with.
    pub with: String,

    /// Additional arguments supplied to the script.
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
pub(crate) struct Pack {
    #[serde(default)]
    pub capabilities: Capabilities,

    #[serde(default)]
    pub bp: PackBP,

    #[serde(default)]
    pub rp: PackRP,

    #[serde(default)]
    pub sp: PackSP,

    #[serde(default)]
    pub wt: PackWT,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
pub(crate) struct PackBP {
    /// The name of the behavior pack.
    #[serde(default)]
    pub name: Localized<String>,

    /// The name of the behavior pack.
    #[serde(default)]
    pub description: Localized<String>,

    #[serde(rename = "type")]
    pub kind: ModuleType,

    /// Define extra dependencies. Note that the behavior pack and the resource
    /// pack definined in the same project will depend on each other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
pub(crate) struct PackRP {
    /// The name of the behavior pack.
    #[serde(default)]
    pub name: Localized<String>,

    /// The name of the behavior pack.
    #[serde(default)]
    pub description: Localized<String>,

    /// Define extra dependencies. Note that the behavior pack and the resource
    /// pack definined in the same project will depend on each other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
pub(crate) struct PackSP {
    /// The name of the behavior pack.
    #[serde(default)]
    pub name: Localized<String>,

    /// The name of the behavior pack.
    #[serde(default)]
    pub description: Localized<String>,

    /// Define extra dependencies. Note that the behavior pack and the resource
    /// pack definined in the same project will depend on each other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_field)
)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct PackWT {
    /// The name of the behavior pack.
    #[serde(default)]
    pub name: Localized<String>,

    /// The name of the behavior pack.
    #[serde(default)]
    pub description: Localized<String>,

    /// The version of the base game your world template requires.
    /// This field is required when building a world pack.
    #[serde(default)]
    pub base_game_version: Option<Version>,

    /// This option will generate a random seed every time a template
    /// is loaded and allow the player to change the seed before
    /// creating a new world.
    #[serde(default)]
    pub allow_random_seed: Option<bool>,

    /// This option is required for any world templates. This will lock
    /// the player from modifying the options of the world.
    #[serde(default)]
    pub lock_template_options: Option<bool>,

    /// Define extra dependencies. Note that the behavior pack and the resource
    /// pack definined in the same project will depend on each other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}
