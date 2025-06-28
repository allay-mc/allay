//! Utilities related to plugins.

use rhai::packages::Package as _;
#[cfg(feature = "config-schema")]
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

use crate::project::Profile;

/// Events for triggering plugins.
#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum Hook {
    /// The time right before files are bundled into an `mcaddon`.
    #[default]
    BeforeBuild,

    /// The time right after files are bundled into an `mcaddon`.
    AfterBuild,
}

pub struct PluginContext {
    /// The name of the plugin.
    pub name: Option<String>,

    /// The profile in which the plugin is invoked in.
    pub profile: Profile,

    /// The project root.
    pub root: PathBuf,

    /// Additional configuration (usually passed by the user via the `[env]` section in the
    /// configuration file).
    pub additional_env: BTreeMap<String, String>,
}

/// Creates a new [Rhai engine][rhai::Engine] that can be used to evaluate filters.
pub fn engine(envs: &BTreeMap<String, String>) -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine.disable_symbol("eval");

    let fs_package = rhai_fs::FilesystemPackage::new();
    fs_package.register_into_engine(&mut engine);

    let mut user_env = rhai::Map::new();
    for (key, value) in env::vars() {
        user_env.insert(key.into(), value.into());
    }
    for (key, value) in envs {
        user_env.insert(key.into(), value.into());
    }

    let mut allay_module = rhai::Module::new();
    allay_module.set_var("os", env::consts::OS);
    allay_module.set_var("env", user_env);
    rhai::FuncRegistration::new("is_command")
        .set_into_module(&mut allay_module, |command: &str| which(command).is_ok());

    engine.register_static_module("allay", allay_module.into());

    engine
}

/// Provides an environment for a command useful for Allay plugins and filters.
pub fn apply_environment(command: &mut Command, context: &PluginContext) {
    command.env(
        "ALLAY_PROFILE",
        match context.profile {
            Profile::Debug => "debug",
            Profile::Release => "release",
        },
    );
    command.env("ALLAY_VERSION", crate::VERSION);
    command.env("ALLAY_PREBUILD", crate::paths::project::prebuild_for_profile(&context.root, context.profile));
    command.env("ALLAY_PROJECT_ROOT", &context.root);
    if let Some(name) = &context.name {
        command.env("ALLAY_PLUGIN_NAME", name);
    }
    // TODO: more
    command.envs(&context.additional_env);
}
