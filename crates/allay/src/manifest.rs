//! Interface for Minecraft's manifest format.manife

use crate::{
    config::{self as cfg, BehaviorPackModuleKind},
    localization, lock, project, Pack,
};
use semver::Version as SemVer;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fmt};
use uuid::Uuid;

// TODO: test if skin pack manifest (v1) is compatible this way

fn convert_subpacks(config_subpacks: &BTreeMap<String, cfg::Subpack>) -> Vec<Subpack> {
    config_subpacks
        .iter()
        .map(|(folder_name, subpack)| Subpack {
            name: localization::keys::subpack_name(&folder_name.replace("=", "")),
            memory_tier: subpack.memory_tier,
            folder_name: folder_name.to_string(),
        })
        .collect()
}

/// An error that occured while building the manifest.
#[derive(Debug, Clone, Copy)]
pub enum BuildError {
    /// Trying to build manifest for a pack whose UUIDs are not existant.
    MissingUuids(Pack),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingUuids(pack) => format!("missing uuid for pack {}", pack),
            }
        )
    }
}

impl Error for BuildError {}

/// Builds the manifest for a pack.
///
/// If the pack is [`Pack::WorldTemplate`], then the configuration
/// must contain [`cfg::Config::wt`].
pub fn build(config: &cfg::Config, pack: &Pack, lock: &lock::Lock) -> Result<Manifest, BuildError> {
    Ok(Manifest {
        format_version: match pack {
            Pack::Skin => 1,
            _ => 2,
        },
        capabilities: config
            .capabilities
            .iter()
            .map(|c| <cfg::Capability as Into<Capability>>::into(*c))
            .collect(),
        dependencies: match pack {
            Pack::Behavior => {
                let mut deps: Vec<Dependency> = config
                    .bp
                    .dependencies
                    .iter()
                    .map(|dep| dep.clone().into())
                    .collect();
                if let Some(dep) = project::internal_dependencies(pack, lock) {
                    deps.push(dep.into());
                }
                deps
            }
            Pack::Resource => {
                let mut deps: Vec<Dependency> = config
                    .rp
                    .dependencies
                    .iter()
                    .map(|dep| dep.clone().into())
                    .collect();
                if let Some(dep) = project::internal_dependencies(pack, lock) {
                    deps.push(dep.into());
                }
                deps
            }
            Pack::Skin => Vec::new(),
            Pack::WorldTemplate => Vec::new(),
        },
        header: Header {
            allow_random_seed: match pack {
                Pack::WorldTemplate => Some(config.wt.as_ref().unwrap().allow_random_seed),
                _ => None,
            },
            base_game_version: match pack {
                Pack::WorldTemplate => Some(config.wt.clone().unwrap().base_game_version.into()),
                _ => None,
            },
            description: localization::keys::pack_description().to_string(),
            lock_template_options: match pack {
                Pack::WorldTemplate => Some(config.wt.as_ref().unwrap().lock_template_options),
                _ => None,
            },
            min_engine_version: Some(config.project.min_engine_version.clone().into()),
            name: localization::keys::pack_name().to_string(),
            pack_scope: match pack {
                Pack::Resource => Some(config.rp.scope.into()),
                _ => None
            },
            uuid: lock
                .uuid_table
                .get(pack)
                .ok_or(BuildError::MissingUuids(*pack))?
                .header
                .uuid,
            version: lock
                .uuid_table
                .get(pack)
                .ok_or(BuildError::MissingUuids(*pack))?
                .header
                .version
                .clone(),
        },
        modules: lock
            .uuid_table
            .get(pack)
            .ok_or(BuildError::MissingUuids(*pack))?
            .modules
            .iter()
            .map(|(key, value)| Module {
                description: match key {
                    ModuleKind::Data | ModuleKind::Resources => {
                        Some(localization::keys::pack_description().to_string())
                    }
                    _ => None,
                },
                kind: *key,
                entry: match key {
                    ModuleKind::Script => Some((|| {
                        for module in &config.bp.modules {
                            match &module.kind {
                                BehaviorPackModuleKind::Script(m) => {
                                    return m.entry.to_string_lossy().to_string();
                                }
                                BehaviorPackModuleKind::Data(_) => {}
                            }
                        }
                        unreachable!();
                    })()),
                    _ => None,
                },
                language: match key {
                    ModuleKind::Script => Some(Language::Javascript),
                    _ => None,
                },
                uuid: value.uuid,
                version: value.version.clone(),
            })
            .collect(),
        metadata: Some(Metadata {
            authors: {
                let authors = &config.project.authors;
                if authors.is_empty() {
                    None
                } else {
                    Some(
                        authors
                            .iter()
                            .map(|author| Author(author.to_string()))
                            .collect(),
                    )
                }
            },
            generated_with: Some(BTreeMap::from([(
                "allay".to_string(),
                vec![crate::VERSION.to_string()],
            )])),
            license: config.project.license.clone(),
            product_type: config.project.product_type.map(|value| value.into()),
            url: config.project.url.clone(),
        }),
        subpacks: match pack {
            Pack::Behavior => convert_subpacks(&config.bp.subpacks),
            Pack::Resource => convert_subpacks(&config.rp.subpacks),
            Pack::Skin => Vec::new(),
            Pack::WorldTemplate => Vec::new(),
        },
    })
}

/// The manifest data used by Minecraft for packs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// This defines the current version of the manifest.
    pub format_version: u32,

    /// These are the different features that the pack makes use
    /// of that aren't necessarily enabled by default.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,

    /// Section containing definitions for any other packs or
    /// modules that are required in order for this manifest.json
    /// file to work.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,

    /// Section containing information regarding the name of the
    /// pack, description, and other features that are public facing.
    pub header: Header,

    /// Section containing information regarding the type of
    /// content that is being brought in.
    pub modules: Vec<Module>,

    /// Section containing the metadata about the file such as
    /// authors and licensing information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A list of subpacks that are applied per memory tier.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub subpacks: Vec<Subpack>,
}

/// These are the different features that the pack makes use
/// of that aren't necessarily enabled by default.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// Allows the pack to add, change or replace Chemistry
    /// functionality.
    Chemistry,

    /// Indicates that this pack contains extensions for editing.
    #[serde(rename = "editorExtension")]
    EditorExtension,

    /// Allows HTML files in the pack to be used for custom UI,
    /// and scripts in the pack to call and manipulate custom UI.
    ExperimentalCustomUI,

    /// Indicates that this pack contains Raytracing Enhanced or
    /// Physical Based Materials for rendering.
    Raytraced,

    #[allow(missing_docs)]
    PBR,

    #[allow(missing_docs)]
    ScriptEval,
}

/// Data for a dependency.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// A built-in dependency (e.g. `@minecraft/server`).
    Builtin(BuiltinDependency),

    /// An external dependency.
    External(ExternalDependency),
}

impl From<cfg::Dependency> for Dependency {
    fn from(value: cfg::Dependency) -> Self {
        match value {
            cfg::Dependency::Builtin(dep) => Dependency::Builtin(dep.into()),
            cfg::Dependency::External(dep) => Dependency::External(dep.into()),
        }
    }
}

/// A built-in dependency (e.g, `@minecraft/server`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinDependency {
    /// The name of the module (e.g. `@minecraft/server`).
    pub module_name: String,

    /// The version of the dependency.
    pub version: String,
}

impl From<cfg::BuiltinDependency> for BuiltinDependency {
    fn from(value: cfg::BuiltinDependency) -> Self {
        BuiltinDependency {
            module_name: value.module_name,
            version: value.version,
        }
    }
}

/// An external dependency.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalDependency {
    /// The UUID of the dependency.
    ///
    /// This should match the dependency's UUID defined in the manifest `header` section.
    pub uuid: Uuid,

    /// The version of the dependency.
    pub version: Version,
}

impl From<cfg::ExternalDependency> for ExternalDependency {
    fn from(value: cfg::ExternalDependency) -> Self {
        ExternalDependency {
            uuid: value.uuid,
            version: value.version.into(),
        }
    }
}

/// A version.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Version {
    /// Format version format (e.g. `1.21.80`).
    FormatVersion(SemVer),

    /// Version with three parts.
    Vec3Version((u32, u32, u32)),
}

impl From<cfg::Version> for Version {
    fn from(value: cfg::Version) -> Self {
        match value {
            cfg::Version::FormatVersion(ver) => Version::FormatVersion(ver),
            cfg::Version::Vec3Version(ver) => Version::Vec3Version(ver),
        }
    }
}

/// Section containing information regarding the name of the
/// pack, description, and other features that are public facing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    /// This option is required for any world templates. This will
    /// allow the player to use a random seed when creating a new
    /// world from your template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_random_seed: Option<bool>,

    /// This is the version of the base game your world template
    /// requires, specified as
    /// [majorVersion, minorVersion, revision]. We use this to
    /// determine what version of the base game resource and
    /// behavior packs to apply when your content is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_version: Option<Version>,

    /// This is a short description of the pack. It will appear in
    /// the game below the name of the pack. We recommend keeping
    /// it to 1-2 lines.
    pub description: String,

    /// This option is required for any world templates. This
    /// will lock the player from modifying the options of the
    /// world.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_template_options: Option<bool>,

    /// This is the minimum version of the game that this pack was
    /// written for. This is a required field for resource and
    /// behavior packs. This helps the game identify whether any
    /// backwards compatibility is needed for your pack. You should
    /// always use the highest version currently available when
    /// creating packs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_engine_version: Option<Version>,

    /// This is the name of the pack as it appears within
    /// Minecraft. This is a required field.
    pub name: String,

    /// This is the scope of the pack. This is only for resource
    /// packs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_scope: Option<PackScope>,

    /// This is a special type of identifier that uniquely
    /// identifies this pack from any other pack.
    pub uuid: Uuid,

    /// This is the version of your pack in the format
    /// [majorVersion, minorVersion, revision].
    pub version: Version, // TODO: or semver
}

/// This is the scope of the pack. This is only for resource
/// packs.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum PackScope {
    #[allow(missing_docs)]
    Global,

    #[allow(missing_docs)]
    World,

    #[allow(missing_docs)]
    #[default]
    Any,
}

impl From<cfg::Scope> for PackScope {
    fn from(value: cfg::Scope) -> Self {
        match value {
            cfg::Scope::Global => Self::Global,
            cfg::Scope::World => Self::World,
            cfg::Scope::Any => Self::Any,
        }
    }
}

/// Section containing information regarding the type of content
/// that is being brought in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// This is a short description of the module. This is not
    /// user-facing at the moment but is a good place to remind
    /// yourself why the module is defined,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// This is the type of the module.
    #[serde(rename = "type")]
    pub kind: ModuleKind,

    /// The programming language to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// This is a unique identifier for the module in the same
    /// format as the pack's UUID in the header. This should be
    /// different from the pack's UUID, and different for every
    /// module.
    pub uuid: Uuid,

    /// This is the version of your pack in the format
    /// [majorVersion, minorVersion, revision]. The version number
    /// is used when importing a pack that has been imported before.
    /// The new pack will replace the old one if the version is
    /// higher, and ignored if it's the same or lower.
    pub version: Version, // TODO: or semver

    /// The javascript entry point for tests, only works if types
    /// has been set to `javascript`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
}

/// The kind of module.
#[derive(Copy, Clone, Debug, Serialize, Hash, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleKind {
    #[allow(missing_docs)]
    Resources,

    #[allow(missing_docs)]
    Data,

    #[allow(missing_docs)]
    ClientData,

    #[allow(missing_docs)]
    Interface,

    #[allow(missing_docs)]
    WorldTemplate,

    #[allow(missing_docs)]
    Javascript,

    #[allow(missing_docs)]
    Script,
}

impl fmt::Display for ModuleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Resources => "resources",
                Self::Data => "data",
                Self::ClientData => "client_data",
                Self::Interface => "interface",
                Self::WorldTemplate => "world_template",
                Self::Javascript => "javascript",
                Self::Script => "script",
            }
        )
    }
}

/// The language used for scripting.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[allow(missing_docs)]
    Javascript,
}

/// Section containing the metadata about the file such as authors
/// and licensing information.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    /// Name of the author(s) of the pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,

    /// A list of tools and their version that have modified this
    /// pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_with: Option<BTreeMap<String, Vec<String>>>,

    /// The license of the pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// The type of product this pack is. This is used to
    /// determine how the pack is displayed in the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,

    /// The home website of your pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// An author of a pack.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author(pub String);

/// The type of product this pack is. This is used to determine
/// how the pack is displayed in the store.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    #[allow(missing_docs)]
    AddOn,
}

impl From<cfg::ProductType> for ProductType {
    fn from(value: cfg::ProductType) -> Self {
        match value {
            cfg::ProductType::AddOn => Self::AddOn,
        }
    }
}

/// A subpack.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Subpack {
    /// This represents the folder name located in "subpacks"
    /// folder. When user select this resolution Minecraft loads
    /// the content inside the folder.
    pub folder_name: String,

    /// This is the name of the pack resolution. This lets user
    /// know what resolution they are choosing.
    pub name: String,

    /// This creates a requirement on the capacity of memory
    /// needed to select the resolution. Each tier increases
    /// memory requirement by 256 MB.
    pub memory_tier: u8,
}
