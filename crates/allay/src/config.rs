//! Configuration for Allay projects.

// TODO: change version as strings to semver

use crate::{
    localization::{self, Language, OptionallyLocalized},
    manifest, plugin::Hook,
};
#[cfg(feature = "config-schema")]
use schemars::JsonSchema;
use semver::Version as SemVer;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    path::PathBuf,
};
use uuid::Uuid;

// Needed for serde's `default_value`.
const fn return_true() -> bool {
    true
}

/// A filter string (e.g, `(env("FOO") ?? "1") == "1"`).
pub type Filter = String;

#[cfg(feature = "config-schema")]
fn any(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    schemars::schema::Schema::Bool(true)
}

/// The configuration of an Allay project.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(rename = "$schema")]
    #[serde(default)]
    /// Optional reference to JSON schema.
    ///
    /// This is not used by Allay but is useful when working with
    /// IDEs for example.
    pub schema: Option<String>,

    /// The metadata of the Allay project.
    pub project: Project,

    /// Localization related options.
    pub localization: Localization,

    /// Section to define static environment variables.
    ///
    /// These can be used as feature flags for instance.
    #[serde(default)]
    pub env: BTreeMap<String, String>,

    /// Configurations for the build process.
    #[serde(default)]
    pub build: Build,

    /// Plugins run during the build process.
    ///
    /// Plugins are executed in order in which they are specified.
    #[serde(rename = "plugin")]
    #[serde(default)]
    pub plugins: Vec<Plugin>,

    /// Behavior pack specific configuration.
    #[serde(rename = "BP")]
    #[serde(default)]
    pub bp: BP,

    /// Resource pack specific configuration.
    #[serde(rename = "RP")]
    #[serde(default)]
    pub rp: RP,

    /// Skin pack specific configuration.
    #[serde(rename = "SP")]
    #[serde(default)]
    pub sp: SP,

    /// World template specific configuration.
    #[serde(rename = "WT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wt: Option<WT>,

    /// Section to enable certain capabilities.
    #[serde(default)]
    pub capabilities: Vec<Capability>,
}

/// Localization related configuration.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Localization {
    /// Controls whether to generate translations or not.
    #[serde(default = "return_true")]
    pub generate_translations: bool,

    /// Custom language groups.
    #[serde(default)]
    pub groups: localization::LanguageGroups,

    /// The primary language used for localization.
    ///
    /// Missing localization will fall back to values of this language if present.
    #[serde(default)]
    pub primary_language: Language,
}

impl Default for Localization {
    fn default() -> Self {
        Self {
            generate_translations: true,
            groups: localization::LanguageGroups::default(),
            primary_language: Default::default(),
        }
    }
}

/// Configuration for certain capabilities.
#[derive(Copy, Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum Capability {
    #[allow(missing_docs)]
    Raytraced,
    #[allow(missing_docs)]
    PBR,
    #[allow(missing_docs)]
    ScriptEval,
    #[allow(missing_docs)]
    EditorExtension,
    #[allow(missing_docs)]
    ExperimentalCustomUI,
    #[allow(missing_docs)]
    Chemistry,
}

impl From<Capability> for manifest::Capability {
    fn from(value: Capability) -> Self {
        match value {
            Capability::Raytraced => manifest::Capability::Raytraced,
            Capability::PBR => manifest::Capability::PBR,
            Capability::ScriptEval => manifest::Capability::ScriptEval,
            Capability::EditorExtension => manifest::Capability::EditorExtension,
            Capability::ExperimentalCustomUI => manifest::Capability::ExperimentalCustomUI,
            Capability::Chemistry => manifest::Capability::Chemistry,
        }
    }
}

/// Metadata for the project.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Project {
    /// The name of the project.
    pub name: OptionallyLocalized<String>,

    /// The description of the project.
    pub description: OptionallyLocalized<String>,

    /// The version of the project.
    pub version: Version,

    /// The authors of the project.
    #[serde(default)]
    pub authors: Vec<Author>,

    /// SPDX license identifier of the license this project is
    /// licensed under.
    pub license: Option<String>,

    /// URL to the homepage of your project.
    pub url: Option<String>,

    /// Minimum version of the game the pack is compatible with.
    pub min_engine_version: Version,

    /// The product type.
    #[serde(default)]
    pub product_type: Option<ProductType>,
}

/// Metadata for an author.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Author {
    /// The name of the author.
    ///
    /// This could be the full name, the partial name, or an artist
    /// name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allay::config::Author;
    /// let author1 = Author {
    ///   name: "John Doe".to_string(),
    ///   email: Some("john.doe@example.org".to_string()),
    /// };
    /// let author2 = Author {
    ///   name: "John".to_string(),
    ///   email: Some("john.doe@example.org".to_string()),
    /// };
    /// let author3 = Author {
    ///   name: "jdcodes".to_string(),
    ///   email: Some("john.doe@example.org".to_string()),
    /// };
    /// ```
    pub name: String,

    /// The email of the author (e.g. `john.doe@example.org`).
    pub email: Option<String>,
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.email {
                Some(email) => format!("{} <{}>", self.name, email),
                None => self.name.to_string(),
            }
        )
    }
}

/// Build related configuration.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Build {
    /// Whether to build in debug or release mode.
    ///
    /// This option can be overridden from the CLI.
    #[serde(default = "return_true")]
    pub debug: bool,

    /// Whether to syncronize the add-on.
    ///
    /// This means that for example the resource pack is stored appropiately in the `com_mojang`
    /// directory and updated accordingly on each build.
    #[serde(default = "return_true")]
    pub sync: bool,

    /// Paths to directories or files that should trigger a rebuild when changed.
    /// `src` and `allay.toml` may not be listed as they trigger a rebuild by default.
    ///
    /// The `watch` command utilizes these paths as well for rebuilds..
    #[serde(default)]
    pub extra_watch: Vec<PathBuf>,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            debug: true,
            sync: true,
            extra_watch: Default::default(),
        }
    }
}

/// Configuration for a singular plugin.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Plugin {
    /// An optional name used to identify the plugin.
    #[serde(default)]
    pub name: Option<String>,

    /// The program to run.
    pub run: String,

    /// Arguments passed to the executable.
    #[serde(default)]
    pub args: Vec<String>,

    /// Options passed as JSON as the last argument to th executable.
    #[serde(default)]
    #[cfg_attr(feature = "config-schema", schemars(schema_with = "any"))]
    pub options: Option<toml::Value>,

    /// A filter that can be used to prevent a plugin from running.
    #[serde(default)]
    pub when: Option<Filter>,

    /// The event that triggers the plugin.
    #[serde(default)]
    pub hook: Hook,

    /// Whether the plugin is "pure".
    ///
    /// A plugin is considered pure when changed source files will
    /// not affect the output of the plugin. So if a plugin for
    /// example converts all `.jpg` images into `.png` images it
    /// will yield the same results when the `.jpg` files have not
    /// been changed. An example for a plugin that is not pure could
    /// be one that creates an `mcfunction` that prints the time
    /// when the project was build. The time must be determined
    /// whenever the project is built.
    ///
    /// Pure plugins will not run if the source has not been
    /// changed. This may improve the build time.
    #[serde(default)]
    pub pure: bool,

    /// Whether the plugin should be run in parallel to other plugins.
    ///
    /// Detaching is useful for plugins that don't depend on other
    /// plugins and are time consuming.
    #[serde(default)]
    pub detached: bool,
}

/// The targeted context for the pack.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    /// Indicating that this pack is intended to be added to players' worlds. 
    AddOn,
}

/// The valid scope of a resource pack.
#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    /// The resource pack can only be added in the context of a world.
    World,

    /// The resource pack can only be added in the context of the game.
    Global,

    /// The resource pack can be added in any context.
    #[default]
    Any,
}

/// Behavior pack specific configuration.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct BP {
    /// Whether to use the `manifest.json` file in the `src/BP/`
    /// directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to use the `pack_icon.png` file in the `src/BP/`
    /// directory instead of using the one in the project's root
    /// or, if not present, using a default icon.
    #[serde(default)]
    pub custom_pack_icon: bool,

    /// Override the name for the behavior pack.
    ///
    /// By default the name is adapted from the `project` section.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override the description for the resource pack.
    ///
    /// By default the description is adapted from the `project`
    /// section.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// Section containing information regarding the type of
    /// content that is being brought in.
    #[serde(default)]
    #[serde(rename = "module")]
    pub modules: Vec<BehaviorPackModule>,

    /// Specify dependencies apart from the packs created in the
    /// project.
    ///
    /// Allay handles dependencies between the resource
    /// and behavior pack that exist within the project.
    #[serde(default)]
    #[serde(rename = "dependency")]
    pub dependencies: Vec<Dependency>,

    /// Specify subpacks used within the behavior pack.
    ///
    /// Each key represents the folder name for the subpack.
    #[serde(default)]
    pub subpacks: BTreeMap<String, Subpack>,
}

/// Resource pack specific configuration.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct RP {
    /// Whether to use the `manifest.json` file in the `src/BP/`
    /// directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to use the `pack_icon.png` file in the `src/RP/`
    /// directory instead of using the one in the project's root
    /// or, if not present, using a default icon.
    #[serde(default)]
    pub custom_pack_icon: bool,

    /// Override the name for the behavior pack.
    ///
    /// By default the name is adapted from the `project` section.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override the description for the resource pack.
    ///
    /// By default the description is adapted from the `project`
    /// section.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// Section containing information regarding the type of
    /// content that is being brought in.
    #[serde(default)]
    #[serde(rename = "module")]
    pub modules: Vec<ResourcePackModule>,

    /// Specify dependencies apart from the packs created in the
    /// project.
    ///
    /// Allay handles dependencies between the resource
    /// and behavior pack that exist within the project.
    #[serde(default)]
    #[serde(rename = "dependency")]
    pub dependencies: Vec<Dependency>,

    /// Specify subpacks used within the resource pack.
    ///
    /// Each key represents the folder name for the subpack.
    #[serde(default)]
    pub subpacks: BTreeMap<String, Subpack>,

    /// The scope of the resource pack.
    #[serde(default)]
    pub scope: Scope,
}

/// Skin pack specific configuration.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct SP {
    /// Whether to use the `manifest.json` file in the `src/BP/`
    /// directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// Whether to use the `pack_icon.png` file in the `src/SP/`
    /// directory instead of using the one in the project's root
    /// or, if not present, using a default icon.
    #[serde(default)]
    pub custom_pack_icon: bool,

    /// Override the name for the behavior pack.
    ///
    /// By default the name is adapted from the `project` section.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override the description for the resource pack.
    ///
    /// By default the description is adapted from the `project`
    /// section.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// Specify dependencies apart from the packs created in the
    /// project.
    ///
    /// Allay handles dependencies between the resource
    /// and behavior pack that exist within the project.
    #[serde(default)]
    #[serde(rename = "dependency")]
    pub dependencies: Vec<Dependency>,

    /// Specify subpacks used within the behavior pack.
    ///
    /// Each key represents the folder name for the subpack.
    #[serde(default)]
    pub subpacks: BTreeMap<String, Subpack>,
}

/// World template specific configuration.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct WT {
    /// Whether to use the `manifest.json` file in the `src/BP/`
    /// directory instead of generating one.
    #[serde(default)]
    pub custom_manifest: bool,

    /// This will allow the player to use a random seed when
    /// creating a new world from your template.
    pub allow_random_seed: bool,

    /// This is the version of the base game your world template
    /// requires, specified as
    /// [majorVersion, minorVersion, revision]. We use this to
    /// determine what version of the base game resource and
    /// behavior packs to apply when your content is used.
    pub base_game_version: Version,

    /// This option is required for any world templates.
    /// This will lock the player from modifying the options of the world.
    #[serde(default)]
    pub lock_template_options: bool,

    /// Override the name for the behavior pack.
    ///
    /// By default the name is adapted from the `project` section.
    #[serde(default)]
    pub name: Option<OptionallyLocalized<String>>,

    /// Override the description for the resource pack.
    ///
    /// By default the description is adapted from the `project`
    /// section.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,
}

/// Configuration for a module for the behavior pack.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub struct BehaviorPackModule {
    /// This is a short description of the module. This is not
    /// user-facing at the moment but is a good place to remind
    /// yourself why the module is defined.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// The version number is used when importing a pack that has
    /// been imported before. The new pack will replace the old
    /// one if the version is higher, and ignored if it's the same
    /// or lower.
    pub version: Version,

    /// This is the type of the module.
    #[serde(flatten)]
    pub kind: BehaviorPackModuleKind,
}

/// The kind of a behavior pack module.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum BehaviorPackModuleKind {
    /// Sets the module as a "data" module.
    Data(DataModule),

    /// Sets the module as a "script" module.
    Script(ScriptModule),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
/// A "data" module.
pub struct DataModule;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
/// A "script" module.
pub struct ScriptModule {
    /// The JavaScript scripts,
    pub entry: PathBuf,
}

/// Configuration for a module for the behavior pack.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub struct ResourcePackModule {
    /// This is a short description of the module. This is not
    /// user-facing at the moment but is a good place to remind
    /// yourself why the module is defined.
    #[serde(default)]
    pub description: Option<OptionallyLocalized<String>>,

    /// The version number is used when importing a pack that has
    /// been imported before. The new pack will replace the old
    /// one if the version is higher, and ignored if it's the same
    /// or lower.
    pub version: Version,

    /// This is the type of the module.
    #[serde(flatten)]
    pub kind: ResourcePackModuleKind,
}

/// The kind of a behavior pack module.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum ResourcePackModuleKind {
    #[allow(missing_docs)]
    Resources,
}

/// Data for a dependency.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum Dependency {
    /// A built-in dependency (e.g. `@minecraft/server`).
    Builtin(BuiltinDependency),

    /// An external dependency.
    External(ExternalDependency),
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (id, ver) = match self {
            Self::Builtin(dep) => (&dep.module_name, &dep.version),
            Self::External(dep) => (&dep.uuid.to_string(), &dep.version.to_string()),
        };
        write!(f, "{} ({})", id, ver)
    }
}

/// A built-in dependency (e.g, `@minecraft/server`).
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct BuiltinDependency {
    /// The name of the module (e.g. `@minecraft/server`).
    pub module_name: String,

    /// The version of the dependency.
    pub version: String,
}

/// An external dependency.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalDependency {
    /// The UUID of the dependency.
    ///
    /// This should match the dependency's UUID defined in the manifest `header` section.
    pub uuid: Uuid,

    /// The version of the dependency.
    pub version: Version,

    /// A hinr to identify the dependency.
    pub hint: Option<String>,
}

/// A version.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum Version {
    /// Format version format (e.g. `1.21.80`).
    FormatVersion(SemVer),

    /// Version with three parts.
    Vec3Version((u32, u32, u32)),
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FormatVersion(string) => write!(f, "{}", string),
            Self::Vec3Version((major, minor, patch)) => write!(f, "{major}.{minor}.{patch}"),
        }
    }
}

impl From<manifest::Version> for Version {
    fn from(value: manifest::Version) -> Self {
        match value {
            manifest::Version::FormatVersion(ver) => Version::FormatVersion(ver),
            manifest::Version::Vec3Version(ver) => Version::Vec3Version(ver),
        }
    }
}

/// A subpack.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Subpack {
    /// The display name of the subpack.
    pub name: OptionallyLocalized<String>,

    /// Amount of RAM that a device must have to enable this
    /// subpack. Each memory tier adds 0.25 GB.
    #[serde(default)]
    pub memory_tier: u8,
}
