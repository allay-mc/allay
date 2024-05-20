//! Configuration for an Allay project.

// TODO: `serde(default)`s

use std::{collections::HashMap, path::PathBuf};

use crate::{
    localization::{Language, LanguageGroups, OptionallyLocalized},
    manifest::{BaseGameVersion, Capabilities},
};
use serde::Deserialize;

/// A version string.
pub type Version = String;

/// A filter string (e.g. `os() == "windows" || os = "linux"`).
pub type Filter = String;

/// The overall configuration for an Allay project.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct Config {
    /// Optional reference to JSON schema.
    #[serde(rename = "$schema")]
    #[serde(default)]
    pub schema: Option<String>,

    /// Whether to build in debug mode.
    #[serde(default)]
    pub debug: bool,

    /// Metadata of the Allay project.
    pub project: Project,

    /// Localization options.
    pub localization: Localization,

    /// Define environment variables which can be used by plugins.
    pub env: HashMap<String, String>,

    /// Configuare the build process.
    #[serde(default)]
    pub build: Build,

    /// Plugins are executable which transform the packs as a process of the build.
    ///
    /// Plugins are executed in the order in which they are specified.
    #[serde(default)]
    pub plugin: Vec<Plugin>, // TODO: name `plugins` but serde_rename `plugin`

    /// Behavior Pack specific configuration.
    #[serde(rename = "BP")]
    #[serde(default)]
    pub bp: BP,

    /// Resource Pack specific configuration.
    #[serde(rename = "RP")]
    #[serde(default)]
    pub rp: RP,

    /// Skin Pack specific configuration.
    #[serde(rename = "SP")]
    #[serde(default)]
    pub sp: SP,

    /// World Template specific configuration.
    #[serde(rename = "WT")]
    #[serde(default)]
    pub wt: WT,

    /// Section containing optional features that can be enabled in Minecraft.
    #[serde(default)]
    pub capabilities: Option<Capabilities>,
}

impl Config {
    pub fn from_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct Build {
    /// Directories to watch besides from `src` when using the `watch` command.
    #[serde(default)]
    pub extra_watch_dirs: Vec<PathBuf>,
}

/// Metadata of the Allay project.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct Project {
    /// The name of the project.
    ///
    /// # Examples
    ///
    /// ```toml
    /// [project]
    /// name = "Name for all languages"
    /// ```
    ///
    /// ```toml
    /// [project.name]
    /// en-us = "Name for English"
    /// de-de = "Name for German"
    /// ```
    pub name: OptionallyLocalized<String>,

    /// The decription of the project.
    pub description: OptionallyLocalized<String>,

    /// The version of the project.
    pub version: Version,

    /// The authors of the project.
    #[serde(default)]
    pub authors: Option<Vec<String>>,

    /// SPDX license identifier of the project.
    pub license: Option<String>,

    /// URL to the homepage of your project.
    #[cfg_attr(feature = "config-schema", schemars(url))]
    pub url: Option<String>,

    /// Minimum version of the game the pack is written for.
    pub min_engine_version: Version,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct Localization {
    /// The primary language used to the add-ons.
    ///
    /// This language is the general fallback for unspecified translations.
    pub primary_language: Language,

    #[serde(default)]
    pub groups: LanguageGroups,
}

/// A plugin that transforms the pack.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct Plugin {
    /// An optional name used to identify the plugin.
    pub name: Option<String>,

    /// The executable or first argument when combined with [`Plugin::with`].
    ///
    /// # Examples
    ///
    /// ```toml
    /// [[plugin]]
    /// run = "myexe"
    /// ```
    ///
    /// ```toml
    /// [[plugin]]
    /// run = "script.py"
    /// with = "python3"
    /// ```
    pub run: String,

    /// An executable usually combined with [`Plugin::run`].
    ///
    /// This is normally the name of an interpreter such as `python3` or `ruby` which runs the appropiate
    /// scripts.
    pub with: Option<String>,

    /// Arguments passed to the executable.
    #[serde(default, flatten)]
    pub args: PluginArgs,

    /// A filter which decides whether the plugin should be run.
    ///
    /// # Examples
    ///
    /// ```toml
    /// [[plugin]]
    /// run = "hello"
    /// when = "any(os = linux, os = android)"
    /// ```
    pub when: Option<Filter>,

    /// Aborts further build process when the plugin run unsuccessful.
    #[serde(default)]
    pub panic: bool,
}

#[cfg(feature = "config-schema")]
fn any(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    schemars::schema::Schema::Bool(true)
}

/// Arguements passed to the executable.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub enum PluginArgs {
    /// Passes data as a JSON object to the executable.
    ///
    /// # Examples
    ///
    /// ```toml
    /// [[plugin]]
    /// run = "scripts/hello.rb"
    /// options = {foo = "bar"}
    /// ```
    ///
    /// <div class="warning">
    /// Note that the arguments are passed as is meaning you cannot make use of shell-specific features like
    /// environment variables or glob patterns.
    /// </div>
    #[cfg_attr(feature = "config-schema", schemars(schema_with = "any"))]
    Options(toml::Value),

    /// Passes the arguments to the executable.
    ///
    /// # Examples
    ///
    /// ```toml
    /// [[plugin]]
    /// run = "ruby"
    /// args = ["-w", "scripts/yaml_to_json.rb"]
    /// ```
    Args(Vec<String>),
}

impl Default for PluginArgs {
    fn default() -> Self {
        Self::Args(Vec::new())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub enum BehaviorPackType {
    #[default]
    Data,
    Script,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct Dependency {
    #[serde(flatten)]
    pub id: Identifier,

    /// The version of the dependency.
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub enum Identifier {
    /// The name of the dependency to use (e.g. `@minecraft/server`).
    ModuleName(String),

    /// The UUID of the dependency to use.
    Uuid(String),
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct BP {
    /// Whether to use the `manifest.json` file in the `BP` directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to use the `pack_icon.png` file in the `BP` directory instead of generating one.
    #[serde(default)]
    pub custom_pack_icon: bool,

    /// Override name for behavior pack.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override description for behavior pack.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// The type of the behavior pack.
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: BehaviorPackType,

    /// Define extra dependencies.
    ///
    /// Note that the behavior pack and the resource pack definined in the same project will depend on each
    /// other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct RP {
    /// Whether to use the `manifest.json` file in the `RP` directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to use the `pack_icon.png` file in the `RP` directory instead of generating one.
    #[serde(default)]
    pub custom_pack_icon: bool,

    /// Override name for behavior pack.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override description for behavior pack.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// Define extra dependencies.
    ///
    /// Note that the behavior pack and the resource pack definined in the same project will depend on each
    /// other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct SP {
    /// Whether to use the `manifest.json` file in the `SP` directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to use the `pack_icon.png` file in the `SP` directory instead of generating one.
    #[serde(default)]
    pub custom_pack_icon: bool,

    /// Override name for behavior pack.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override description for behavior pack.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// Define extra dependencies.
    ///
    /// Note that the behavior pack and the resource pack definined in the same project will depend on each
    /// other by default.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
pub struct WT {
    /// Whether to use the `manifest.json` file in the `WT` directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to exclude the resource pack from the world template.
    #[serde(default)]
    pub exclude_rp: bool,

    /// Whether to exclude the behavior pack from the world template.
    #[serde(default)]
    pub exclude_bp: bool,

    #[serde(default)]
    pub allow_random_seed: bool,

    #[serde(default)]
    pub base_game_version: BaseGameVersion,

    /// Override name for behavior pack.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override description for behavior pack.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,
}
