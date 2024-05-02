//! `manifest.json` generator.
//!
//! # References
//!
//! - <https://learn.microsoft.com/en-us/minecraft/creator/reference/content/addonsreference/examples/addonmanifest?view=minecraft-bedrock-stable>

use crate::config;
use crate::config::BehaviorPackType;
use crate::Error;
use crate::Pack;
use crate::Project;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// NOTE: The official documentation states that `version` keys may alternatively be SemVer strings but that
//       does not seem to work. The format `[majorVersion, minorVersion, revision]` works however. The
//       `generated_with` section in the metadata however **only** accepts strings (even though metadata
//       appears to be ignored by Minecraft anyways).
pub type Version = [u8; 3];

fn version_from_string(s: &str) -> Version {
    let version: Vec<u8> = s
        .split(".")
        .take(3)
        .map(|x| x.parse::<u8>().expect("not a number")) // TODO: better error handling
        .collect();
    Version::try_from(version).expect("invalid version") // TODO: better error handling
}

/// The manifest format used to "package" Minecraft add-ons.
#[derive(Debug, Serialize)]
pub struct Manifest {
    /// The syntax version used in the manifest file. This may be 1 for skin
    /// packs or 2 for resource, behavior, and world packs.
    pub format_version: u8,

    /// Section containing information regarding the name of the pack,
    /// description, and other features that are public facing.
    pub header: Header,

    /// Section containing information regarding the type of content that is
    /// being brought in.
    pub modules: Option<Vec<Module>>,

    /// Section containing definitions for any other packs that are required
    /// in order for this manifest.json file to work.
    ///
    /// # References
    ///
    /// - <https://learn.microsoft.com/en-us/minecraft/creator/documents/behaviorpack?view=minecraft-bedrock-stable#create-the-dependency>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,

    /// Section containing optional features that can be enabled in Minecraft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    /// Section containing the metadata about the file such as authors and
    /// licensing information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl Manifest {
    pub fn build(pack: Pack, project: Project) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Manifest {
            format_version: 2,
            header: Header {
                allow_random_seed: match pack {
                    Pack::WorldTemplate => Some(project.config.wt.allow_random_seed),
                    _ => None,
                },
                base_game_version: if pack == Pack::WorldTemplate {
                    Some(project.config.wt.base_game_version)
                } else {
                    None
                },
                description: String::from("pack.description"),
                lock_template_options: None,
                min_engine_version: Some(version_from_string(
                    &project.config.project.min_engine_version,
                )),
                // min_engine_version: Some((1, 19, 0)),
                name: String::from("pack.name"),
                uuid: project
                    .uuids
                    .of(&pack)
                    .header
                    .ok_or(Error::MissingUuid(pack))?
                    .to_string(),
                version: version_from_string(&project.config.project.version),
                // version: (1, 0, 0),
            },
            modules: Some(vec![Module {
                description: String::from("pack.description"),
                kind: match &pack {
                    Pack::Behavior => project.config.bp.kind.into(),
                    Pack::Resource => ModuleType::Resources,
                    Pack::Skin => ModuleType::SkinPack,
                    Pack::WorldTemplate => ModuleType::WorldTemplate,
                },
                language: match (pack, project.config.bp.kind) {
                    (Pack::Behavior, BehaviorPackType::Script) => Some(Language::JavaScript),
                    _ => None,
                },
                uuid: project
                    .uuids
                    .of(&pack)
                    .module
                    .ok_or(Error::MissingUuid(pack))?
                    .to_string(),
                version: version_from_string(&project.config.project.version),
                // version: (1, 0, 0),
            }]),
            dependencies: match pack {
                Pack::Behavior => {
                    let mut deps: Vec<Dependency> = Vec::new();
                    deps.extend(project.config.bp.dependencies.iter().map(|dep| dep.into()));
                    if Pack::Resource.exists() {
                        deps.push(Dependency {
                            uuid: project
                                .uuids
                                .rp
                                .header
                                .ok_or(Error::MissingUuid(Pack::Resource))?
                                .to_string(),
                            version: version_from_string(&project.config.project.version),
                            // version: (1, 0, 0),
                        });
                    }
                    Some(deps)
                }
                Pack::Resource => {
                    let mut deps: Vec<Dependency> = Vec::new();
                    deps.extend(project.config.rp.dependencies.iter().map(|dep| dep.into()));
                    Some(deps)
                }
                Pack::Skin => None,
                Pack::WorldTemplate => None,
            },
            capabilities: None, // TODO
            metadata: Some(Metadata {
                authors: project.config.project.authors,
                license: project.config.project.license,
                generated_with: Some({
                    let mut map = HashMap::new();
                    map.insert(
                        String::from("allay"),
                        vec![clap::crate_version!().to_string()],
                    );
                    map
                }),
                url: project.config.project.url,
            }),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Header {
    /// This option will generate a random seed every time a template is
    /// loaded and allow the player to change the seed before creating a new
    /// world.
    ///
    /// [None] if not a world pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_random_seed: Option<bool>,

    /// This is the version of the base game your world template requires,
    /// specified as [majorVersion, minorVersion, revision]. We use this to
    /// determine what version of the base game resource and behavior packs
    /// to apply when your content is used.
    ///
    /// [None] if not a world pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_version: Option<BaseGameVersion>,

    /// This is a short description of the pack. It will appear in the game
    /// below the name of the pack. We recommend keeping it to 1-2 lines.
    pub description: String,

    /// This option is required for any world templates. This will lock the
    /// player from modifying the options of the world.
    ///
    /// [None] if not a world pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_template_options: Option<bool>,

    /// This is the minimum version of the game that this pack was written
    /// for. This is a required field for resource and behavior packs. This
    /// helps the game identify whether any backwards compatibility is needed
    /// for your pack. You should always use the highest version currently
    /// available when creating packs.
    pub min_engine_version: Option<Version>,

    /// This is the name of the pack as it appears within Minecraft. This is
    /// a required field.
    pub name: String,

    /// This is a special type of identifier that uniquely identifies this
    /// pack from any other pack. UUIDs are written in the format
    /// xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx where each x is a hexadecimal
    /// value (0-9 or a-f). We recommend using an online service to generate
    /// this and guarantee their uniqueness,
    pub uuid: String,

    /// This is the version of your pack in the format
    /// [majorVersion, minorVersion, revision]. The version number is used when
    /// importing a pack that has been imported before. The new pack will
    /// replace the old one if the version is higher, and ignored if it's the
    /// same or lower.
    pub version: Version,
}

#[derive(Debug, Serialize)]
pub struct Module {
    /// This is a short description of the module. This is not user-facing at
    /// the moment but is a good place to remind yourself why the module is
    /// defined.
    pub description: String,

    /// This is the type of the module. Can be any of the following:
    /// `resources`, `data`, `client_data`, `interface`, `world_template` or
    /// `script`.
    #[serde(rename = "type")]
    pub kind: ModuleType,

    /// Only present if `type` is `script`. This indicates the language in which scripts are written in the
    /// pack. The only supported value is `javascript`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// This is a unique identifier for the module in the same format as the
    /// pack's UUID in the header. This should be different from the pack's
    /// UUID, and different for every module.
    pub uuid: String,

    /// This is the version of the module in the same format as the pack's
    /// version in the header. This can be used to further identify changes
    /// in your pack.
    pub version: Version,
}

/// A reference to an add-on by specifying its UUID and version.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct Dependency {
    /// This is the unique identifier of the pack that this pack depends on.
    /// It needs to be the exact same UUID that the pack has defined in the
    /// header section of its manifest file.
    pub uuid: String,

    /// This is the specific version of the pack that your pack depends on.
    /// Should match the version the other pack has in its manifest file.
    pub version: Version,
}

impl From<&config::Dependency> for Dependency {
    fn from(value: &config::Dependency) -> Self {
        Self {
            uuid: match &value.id {
                config::Identifier::ModuleName(_) => todo!("module_name key; use UUID instead"),
                config::Identifier::Uuid(id) => id.to_string(),
            },
            version: version_from_string(&value.version),
            // version: (1, 0, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum Language {
    #[default]
    #[serde(rename = "javascript")]
    JavaScript,
}

/// > **Warning**
/// >
/// > The type of the keys are nowhere documented. I'm assuming these are
/// > boolean values.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct Capabilities {
    /// The pack can add, remove, or modify chemistry behavior.
    pub chemistry: bool,

    /// Indicates that this pack contains extensions for editing.
    #[serde(rename = "editorExtension")]
    pub editor_extension: bool,

    /// The pack can use HTML files to create custom UI, as well as use
    /// or modify the custom UI.
    pub experimental_custom_ui: bool,

    /// The pack uses Ray Tracking functionality and may use custom shaders.
    pub raytraced: bool,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    /// Name of the author(s) of the pack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,

    /// The license of the pack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// This is the tools used to generate a manifest.json file. The tool
    /// names are strings that must be [a-zA-Z0-9_-] and 32 characters maximum.
    /// The tool version number are semver strings for each version that
    /// modified the manifest.json file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_with: Option<HashMap<String, Vec<String>>>,

    /// The home website of your pack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
#[non_exhaustive]
pub enum ModuleType {
    Resources,
    #[default]
    Data,
    ClientData,
    Interface,
    WorldTemplate,
    Script,
    SkinPack,
}

impl From<BehaviorPackType> for ModuleType {
    fn from(value: BehaviorPackType) -> Self {
        match value {
            BehaviorPackType::Data => Self::Data,
            BehaviorPackType::Script => Self::Script,
        }
    }
}

/// In the header of your world template's manifest, you will need to specify the Minecraft version your
/// world template was created for using the `base_game_version` field.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum BaseGameVersion {
    /// If your content is version agnostic (such as a simple survival spawn which
    /// is unlikely to break from future updates), you can forgo locking your
    /// content to a specific version by using a "wildcard": "base_game_version": "*".
    #[default]
    #[serde(rename = "*")]
    Wild,

    #[serde(untagged)]
    Version(usize, usize, usize),
}
