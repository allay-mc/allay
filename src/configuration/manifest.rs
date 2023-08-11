use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct Manifest {
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

#[derive(Debug, Serialize)]
pub(crate) struct Header {
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
    pub base_game_version: Option<(usize, usize, usize)>,

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
    pub min_engine_version: String,

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
    pub version: (usize, usize, usize),
}

#[derive(Debug, Serialize)]
pub(crate) struct Module {
    /// This is a short description of the module. This is not user-facing at
    /// the moment but is a good place to remind yourself why the module is
    /// defined.
    pub description: String,

    /// This is the type of the module. Can be any of the following:
    /// `resources`, `data`, `client_data`, `interface`, `world_template` or
    /// `javascript`.
    #[serde(rename = "type")]
    pub kind: ModuleType,

    /// This is a unique identifier for the module in the same format as the
    /// pack's UUID in the header. This should be different from the pack's
    /// UUID, and different for every module.
    pub uuid: String,

    /// This is the version of the module in the same format as the pack's
    /// version in the header. This can be used to further identify changes
    /// in your pack.
    pub version: (usize, usize, usize),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Dependency {
    /// This is the unique identifier of the pack that this pack depends on.
    /// It needs to be the exact same UUID that the pack has defined in the
    /// header section of its manifest file.
    pub uuid: String,

    /// This is the specific version of the pack that your pack depends on.
    /// Should match the version the other pack has in its manifest file.
    pub version: (usize, usize, usize),
}

/// > **Warning**
/// >
/// > The type of the keys are nowhere documented. I'm assuming these are
/// > boolean values.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Capabilities {
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
pub(crate) struct Metadata {
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

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub(crate) enum ModuleType {
    Resources,
    #[default]
    Data,
    ClientData,
    Interface,
    WorldTemplate,
    JavaScript,
    SkinPack,
}
