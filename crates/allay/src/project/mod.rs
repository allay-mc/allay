//! Project related utilities.

// TODO: rebuild may be required when cli input is changed (e.g. --skip)
// TODO: respect plugin hook

pub mod health;

use crate::{
    config::{self, Config, Plugin},
    localization, lock, manifest, pack, paths,
    plugin::{self, PluginContext},
    Pack,
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use slugify::slugify;
use std::{
    env, fmt, fs,
    io::{self, BufRead, BufReader, Cursor},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime},
};
use uuid::Uuid;
use zip_extensions::zip_create_from_directory;

type Id = Uuid;

const FINGERPRINT: &str = ".allay-fingerprint";

/// Returns dependencies of internal packs that are required for a pack.
pub(crate) fn internal_dependencies(pack: &Pack, lock: &lock::Lock) -> Option<config::Dependency> {
    let opposite_pack = match pack {
        Pack::Behavior => Pack::Resource,
        Pack::Resource => Pack::Behavior,
        _ => {
            return None;
        }
    };
    if let Some(uuid_data) = lock.uuid_table.get(&opposite_pack) {
        let dep = config::Dependency::External(config::ExternalDependency {
            uuid: uuid_data.header.uuid,
            version: uuid_data.header.version.clone().into(),
            hint: None,
        });
        return Some(dep);
    }
    None
}

/// An error that occured while building the project.
#[derive(Clone, Copy, Debug)]
pub struct BuildError;

/// Context used for the build process.
#[derive(Clone, Debug)]
pub struct BuildContext {
    /// The [`Profile`] the project should be built in.
    pub profile: Profile,

    /// Enforces build even if source does not seem to have changed since last build.
    pub force: bool,

    /// Whether to synchronize the add-on in the local Minecraft folders.
    pub sync: bool,

    /// Collection of plugins to skip.
    pub skip_plugins: Vec<String>,
}

impl Default for BuildContext {
    fn default() -> Self {
        Self {
            profile: Profile::Debug,
            force: false,
            sync: true,
            skip_plugins: Vec::new(),
        }
    }
}

impl BuildContext {
    /// Sets attributes according to the [`Config`].
    pub fn with_config(&mut self, config: &Config) -> &mut Self {
        self.with_profile(if config.build.debug {
            Profile::Debug
        } else {
            Profile::Release
        })
    }

    /// Sets the [`Profile`] for the build context.
    pub fn with_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = profile;
        self
    }

    /// Sets `force` for the build context.
    pub fn with_force(&mut self, force: bool) -> &mut Self {
        self.force = force;
        self
    }

    /// Sets `sync` for the build context.
    pub fn with_sync(&mut self, sync: bool) -> &mut Self {
        self.sync = sync;
        self
    }

    /// Adds a plugin to be skipped.
    pub fn skip_plugin(&mut self, plugin_name: String) -> &mut Self {
        self.skip_plugins.push(plugin_name);
        self
    }
}

/// The profile describes in which mode the project should be built in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    /// The debug mode should be used during development.
    Debug,

    /// The release mode should be used when publishing.
    Release,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Debug => "debug",
                Self::Release => "release",
            }
        )
    }
}

/// Finds the matching add-on by the project ID (`.allay-fingerprint`).
///
/// # Parameters
///
/// - `dir` - The directory (for example `.../com.mojang/development_skin_packs`).
/// - `potential_project_name` - The potential project name used by the add-on. This is only used
///   for faster identification.
/// - `project_id` - The ID of the project.
fn find_matching_addon<P>(
    dir: P,
    potential_project_name: &str,
    project_id: &Uuid,
) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let candidate = dir.join(potential_project_name);
    let fingerprint_file = candidate.join(FINGERPRINT);
    match fingerprint_matches(&fingerprint_file, project_id) {
        Ok(true) => {
            return Some(candidate);
        }
        Ok(false) => {
            if fingerprint_file.exists() {
                log::warn!("The add-on {}/{potential_project_name} has {FINGERPRINT} but is doesn't match the project's ID", dir.display());
            } else {
                log::debug!(
                    "The add-on {}/{potential_project_name} doesn't exist",
                    dir.display()
                );
            }
        }
        Err(error) => {
            log::debug!("{}", error);
        }
    };

    match dir.read_dir() {
        Ok(read_dir) => {
            for entry in read_dir.filter_map(|entry| entry.ok()) {
                let p = entry.path();
                if p.is_dir() {
                    let fingerprint_file = p.join(FINGERPRINT);
                    match fingerprint_matches(&fingerprint_file, project_id) {
                        Ok(true) => {
                            return Some(p);
                        }
                        Ok(false) => {
                            if fingerprint_file.exists() {
                                log::debug!("The add-on {} has {FINGERPRINT} but is doesn't match the project's ID", p.display());
                            }
                        }
                        Err(error) => {
                            log::error!("{}", error);
                        }
                    }
                }
            }
        }
        Err(error) => {
            log::warn!(
                "Failed to walk through entries of {}: {}",
                dir.display(),
                error
            );
        }
    };
    None
}

#[allow(clippy::needless_return)] // explicit returns make it clearer in my opinion
fn fingerprint_matches<P>(fingerprint: P, id: &Uuid) -> Result<bool, uuid::Error>
where
    P: AsRef<Path>,
{
    match fs::read(fingerprint) {
        Ok(content) => {
            match Uuid::from_slice(&content) {
                Ok(ref uuid) => {
                    if uuid == id {
                        return Ok(true);
                    } else {
                        return Ok(false);
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            };
        }
        Err(_error) => {
            return Ok(false);
        }
    };
}

/// Fallbacks for missing resources during build.
#[derive(Debug, Clone)]
pub struct Fallbacks {
    /// The default pack icon to use.
    ///
    /// By default, this is the Allay logo.
    pub pack_icon: Vec<u8>,
}

impl Default for Fallbacks {
    fn default() -> Self {
        Fallbacks {
            pack_icon: include_bytes!("fallbacks/pack_icon.png").to_vec(),
        }
    }
}

/// An Allay project.
#[derive(Debug, Clone)]
pub struct Project {
    /// The unique ID of the project.
    ///
    /// Each project has a unique ID (in UUID format).
    id: Id,

    /// The root path of the project.
    ///
    /// This is the directory that contains the configuration file.
    pub root: PathBuf,

    /// The configuration of the project.
    pub config: Config,

    /// The lock of the project.
    pub lock: Option<lock::Lock>,

    /// Fallbacks to use for missing values in a project.
    pub fallbacks: Fallbacks,
}

impl Project {
    /// Loads an existing project.
    ///
    /// This function accepts the path to the root of the project. The path
    /// must point to a directory that contains an `allay.toml` file.
    pub fn load_from_dir(path: &Path) -> Result<Self, InvalidProject> {
        if !path.is_dir() {
            return Err(InvalidProject::IO(io::Error::new(
                io::ErrorKind::Other,
                format!("{} is not a directory", path.display()),
            )));
        }

        let config_file_path = path.join("allay.toml");
        let config_toml =
            fs::read_to_string(config_file_path).map_err(|error| match error.kind() {
                io::ErrorKind::NotFound => InvalidProject::MissingConfigFile,
                _ => InvalidProject::IO(error),
            })?;
        let config: Config =
            toml::from_str(&config_toml).map_err(InvalidProject::InvalidConfigFile)?;

        let lock_file_path = path.join("allay.lock");
        let lock = if lock_file_path.is_file() {
            let lock_toml = fs::read_to_string(lock_file_path)?;
            Some(lock::load(&lock_toml).map_err(InvalidProject::InvalidLockFile)?)
        } else {
            None
        };

        let project_id_file_path = paths::project::project_id(path);
        let project_id_data =
            fs::read(project_id_file_path).map_err(|error| match error.kind() {
                io::ErrorKind::NotFound => InvalidProject::MissingProjectId,
                _ => InvalidProject::IO(error),
            })?;
        let project_id = Uuid::from_slice(&project_id_data)
            .map_err(|_error| InvalidProject::InvalidProjectId(project_id_data))?;

        Ok(Self {
            id: project_id,
            root: path.to_path_buf(),
            config,
            lock,
            fallbacks: Fallbacks::default(),
        })
    }

    /// Loads an existing project if the current working directory is within
    /// a project.
    pub fn load_from_within() -> Result<Self, InvalidProject> {
        let mut current_dir = env::current_dir()?;
        loop {
            let config_file_path = current_dir.join("allay.toml");
            if config_file_path.is_file() {
                return Self::load_from_dir(&current_dir);
            }
            current_dir = match current_dir.parent() {
                Some(dir) => dir.to_path_buf(),
                None => {
                    break;
                }
            };
        }
        Err(InvalidProject::ConfigFileNotFound)
    }

    /// Slugifies the project name.
    #[allow(clippy::expect_used)]
    pub fn slugify_project_name(&self) -> String {
        let project_name = match &self.config.project.name {
            localization::OptionallyLocalized::Localized(translations) => {
                let localizer = localization::fallback_handler_by_groups(
                    self.config.localization.groups.clone(),
                );
                localizer(translations, self.config.localization.primary_language)
                    .expect("TODO: translation should be present")
            }
            localization::OptionallyLocalized::Unlocalized(value) => value,
        };
        slugify!(project_name)
    }

    /// Slugifies the project name with the version, build number and profile.
    pub fn slugify_project_name_full(&self, profile: Profile) -> String {
        format!(
            "{}-{}-{}+{}",
            self.slugify_project_name(),
            profile,
            self.config.project.version,
            self.build_num_for_profile(profile).unwrap_or(0)
        )
    }

    /// Returns the current build number or [`None`] if absent.
    pub fn build_num_for_profile(&self, profile: Profile) -> Option<u32> {
        let last_build_num_path = paths::project::build_num_for_profile(&self.root, profile);
        match fs::read(&last_build_num_path) {
            Ok(content) => {
                let mut reader = Cursor::new(content);
                match reader.read_u32::<BigEndian>() {
                    Ok(num) => Some(num),
                    Err(error) => {
                        log::error!("Failed to get build num; using 0: {}", error);
                        None
                    }
                }
            }
            Err(error) => {
                log::error!("Failed to get num: {}", error);
                None
            }
        }
    }

    /// Builds the project.
    #[allow(clippy::unwrap_in_result)]
    pub fn build(&mut self, context: &BuildContext) -> Result<(), BuildError> {
        // TODO: collect errors and group them into fatal/not fatal errors and report how many
        //       errors occured
        // TODO: create context (`allay explain`) for errors/warnings where context makes sense

        let mut result = Ok(());
        log::info!("Building project");

        log::trace!("{:#?}", &context);
        log::trace!("{:#?}", &self.config);

        let start_time = SystemTime::now();

        let profile = context.profile;
        let sync = context.sync;

        let slugified_name = self.slugify_project_name();

        if !context.force {
            if !health::source_has_changed(self, profile) && self.all_plugins_are_pure() {
                log::info!("Source hasn't changed so not rebuilding");
                return match fs::read(paths::project::last_build_for_profile(&self.root, profile)) {
                    Ok(content) => {
                        let mut reader = Cursor::new(content);
                        match reader.read_u8() {
                            Ok(0) => Err(BuildError),
                            Ok(1) => Ok(()),
                            Ok(byte) => {
                                log::error!("last_build file should be 0 or 1, not {}", byte);
                                Err(BuildError)
                            }
                            Err(error) => {
                                log::error!(
                                    "last_build file should contain exactly one byte: {}",
                                    error
                                );
                                Err(BuildError)
                            }
                        }
                    }
                    Err(error) => {
                        log::error!("Failed to read last_build file: {}", error);
                        Err(BuildError)
                    }
                };
            } else {
                log::trace!("Source has changed or non-pure plugins are used so rebuilding");
            }
        }

        log::debug!("Deleting old prebuild");
        let prebuild_path = paths::project::prebuild_for_profile(&self.root, profile);
        if prebuild_path.exists() {
            if let Err(error) = fs::remove_dir_all(&prebuild_path) {
                log::error!("Failed to delete old prebuild: {}", error);
                result = Ok(());
            };
        } else {
            log::debug!("No old prebuild exists");
        }

        log::trace!("Checking for content in source");
        let mut empty_source = true;
        for pack in pack::packs() {
            if health::source_has_content(self, &pack) {
                log::debug!("Pack {} has content", pack);
                empty_source = false;
            }
        }
        if empty_source {
            log::warn!("No content in `src` so not building");
            return Ok(());
        }

        if health::source_has_content(self, &Pack::WorldTemplate) && self.config.wt.is_none() {
            log::error!(
                "`[WT]` section in allay.toml is required when distributing world template"
            );
            return Err(BuildError);
        }

        log::debug!("Updating/creating lock");
        let updated_lock = lock::update(self);
        if let Err(error) = fs::write(paths::project::lock(&self.root), lock::write(&updated_lock))
        {
            log::error!("Failed to write lock: {}", error);
            result = Err(BuildError);
        }

        log::debug!("Creating prebuild directory");
        if let Err(error) = fs::create_dir_all(&prebuild_path) {
            log::error!("Failed to create prebuild directory: {}", error);
            result = Err(BuildError);
        }

        for pack in pack::packs() {
            if health::source_has_content(self, &pack) {
                if let Err(error) = fs::create_dir(paths::project::prebuild_of_pack_for_profile(
                    &self.root, pack, profile,
                )) {
                    log::error!(
                        "Failed to create prebuild directory for {}: {}",
                        pack,
                        error
                    );
                    result = Err(BuildError);
                }
            }
        }

        log::debug!("Copying source to prebuild");
        for pack in pack::packs() {
            if !health::source_has_content(self, &pack) {
                continue;
            }

            let source = paths::project::source_of_pack(&self.root, pack);
            let dest = paths::project::prebuild_of_pack_for_profile(&self.root, pack, profile);

            let copy_options = fs_extra::dir::CopyOptions::new()
                .copy_inside(true)
                .content_only(true);
            if let Err(error) = fs_extra::dir::copy(source, dest, &copy_options) {
                log::error!("Error while copying to prebuild: {}", error);
                result = Err(BuildError);
            };
        }

        log::debug!("Generating manifests");
        for pack in pack::packs() {
            let custom_manifest: bool = match pack {
                Pack::Behavior => self.config.bp.custom_manifest,
                Pack::Resource => self.config.rp.custom_manifest,
                Pack::Skin => self.config.sp.custom_manifest,
                Pack::WorldTemplate => self.config.wt.as_ref().is_some_and(|wt| wt.custom_manifest),
            };
            if custom_manifest || !health::source_has_content(self, &pack) {
                continue;
            }
            if paths::project::source_of_pack(&self.root, pack)
                .join("manifest.json")
                .is_file()
            {
                log::warn!(
                    "manifest.json of {} will be ignored as `custom-manifest` is not set to `true`",
                    pack
                );
            }
            match manifest::build(&self.config, &pack, &updated_lock) {
                Ok(manifest_data) => {
                    log::trace!("{:#?}", manifest_data);
                    let pack_root =
                        paths::project::prebuild_of_pack_for_profile(&self.root, pack, profile);
                    #[allow(clippy::expect_used)]
                    let manifest_json = match context.profile {
                        Profile::Debug => serde_json::to_string_pretty(&manifest_data),
                        Profile::Release => serde_json::to_string(&manifest_data),
                    }
                    .expect("Failed to represent manifest as JSON");
                    let manifest_path = pack_root.join("manifest.json");
                    if let Err(error) = fs::write(manifest_path, manifest_json) {
                        log::error!("Failed to create manifest: {}", error);
                        result = Err(BuildError);
                    }
                }
                Err(error) => {
                    log::error!("Failed to create manifest: {}", error);
                    result = Err(BuildError);
                }
            };
        }

        log::debug!("Generating pack icons");
        for pack in pack::packs() {
            let custom_pack_icon: bool = match pack {
                Pack::Behavior => self.config.bp.custom_pack_icon,
                Pack::Resource => self.config.rp.custom_pack_icon,
                Pack::Skin => self.config.sp.custom_pack_icon,
                Pack::WorldTemplate => continue,
            };
            if custom_pack_icon || !health::source_has_content(self, &pack) {
                continue;
            }
            if paths::project::source_of_pack(&self.root, pack)
                .join("pack_icon.png")
                .is_file()
            {
                log::warn!("pack_icon.png of {} will be ignored as `custom-pack-icon` is not set to `true`", pack);
            }
            // TODO: the code below is executed in each loop which is not neccessary
            // TODO: when all packs make use of custom pack icon, then we don't need the global one
            let icon_path = paths::project::pack_icon(&self.root);
            let icon = if icon_path.is_file() {
                match fs::read(icon_path) {
                    Ok(content) => content,
                    Err(error) => {
                        log::error!("Failed to read pack_icon.png: {}", error);
                        result = Err(BuildError);
                        continue;
                    }
                }
            } else {
                log::warn!("Missing pack_icon.png at top level");
                self.fallbacks.pack_icon.clone()
            };
            log::trace!("Copying global pack icon");
            let pack_root = paths::project::prebuild_of_pack_for_profile(&self.root, pack, profile);
            if let Err(error) = fs::write(pack_root.join("pack_icon.png"), icon) {
                log::error!("Error while copying pack icon: {}", error);
            }
        }

        if self.config.localization.generate_translations {
            log::debug!("Generating translations");
            for pack in pack::packs() {
                if !health::source_has_content(self, &pack) {
                    continue;
                }
                let texts_dir =
                    paths::project::prebuild_of_pack_for_profile(&self.root, pack, profile)
                        .join("texts");
                if let Err(error) = fs::create_dir_all(&texts_dir) {
                    log::error!("Failed to create texts directory: {}", error);
                    result = Err(BuildError);
                }

                let project_name = &self.config.project.name;
                let project_desc = &self.config.project.description;

                let pack_name = self.config.bp.name.as_ref().unwrap_or(project_name);
                let pack_desc = self.config.bp.description.as_ref().unwrap_or(project_desc);

                let groups = self.config.localization.groups.clone();
                let fallback_handler = &localization::fallback_handler_by_groups(groups);

                for (key, value) in [
                    (localization::keys::pack_name(), pack_name),
                    (localization::keys::pack_description(), pack_desc),
                ] {
                    if let Err(error) = localization::add_translation_with_fallbacks_to_files(
                        &texts_dir,
                        key,
                        value,
                        fallback_handler,
                    ) {
                        log::error!(
                            "Failed to add translation {} -> {:?}: {}",
                            key,
                            value,
                            error
                        );
                        result = Err(BuildError);
                    };
                }

                let languages_data = texts_dir.join("languages.json");
                if languages_data.is_file() {
                    log::warn!("Not generating languages.json because it already exists");
                } else {
                    log::info!("Generating languages.json");
                    match localization::generate_languages_data(texts_dir) {
                        Ok(data) => {
                            let content = match context.profile {
                                Profile::Debug => serde_json::to_string(&data),
                                Profile::Release => serde_json::to_string_pretty(&data),
                            };
                            match content {
                                Ok(content) => {
                                    if let Err(error) = fs::write(languages_data, content) {
                                        log::error!("Failed to write to languages.json: {}", error);
                                    };
                                }
                                Err(error) => {
                                    log::error!("Failed to generate languages.json: {}", error);
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("Failed to generate languages.json: {}", error);
                        }
                    };
                }
            }
        }

        log::debug!("Running plugins");
        let engine = plugin::engine(&self.config.env);
        let mut unhandled_skips = context.skip_plugins.clone();
        for plugin in &self.config.plugins {
            // TODO: prefix log messages with plugin name for easier identifying
            let unnamed_plugin = "<unnamed plugin>".to_string();
            let plugin_name = plugin.name.as_ref().unwrap_or(&unnamed_plugin);

            let mut command_builder = Command::new(&plugin.run);
            command_builder.stdout(Stdio::piped());
            command_builder.stderr(Stdio::piped());
            command_builder.args(&plugin.args);
            if let Some(options_toml) = &plugin.options {
                #[allow(clippy::expect_used)]
                let options_json =
                    serde_json::to_string(options_toml).expect("failed to transform TOML to JSON");
                command_builder.arg(options_json);
            }
            let plugin_context = PluginContext {
                name: plugin.name.clone(),
                profile,
                root: self.root.clone(),
                additional_env: self.config.env.clone(),
            };
            plugin::apply_environment(&mut command_builder, &plugin_context);

            if let Some(expr) = &plugin.when {
                let mut config_skip = false;
                if let Some(name) = &plugin.name {
                    for skip_plugin_name in &context.skip_plugins {
                        if name == skip_plugin_name {
                            config_skip = true;
                            unhandled_skips.retain(|n| n != name);
                        }
                    }
                }
                // TODO: filter should have access to the same environment as the plugin itself
                let filter_skip = match engine.eval_expression::<bool>(expr) {
                    Ok(value) => !value,
                    Err(error) => {
                        log::error!(
                            r#"Failed to evaluate plugin filter for "{}": {}"#,
                            plugin_name,
                            error
                        );
                        result = Err(BuildError);
                        break;
                    }
                };
                let skip = config_skip || filter_skip;
                if skip {
                    log::info!(r#"Skipping plugin "{}""#, plugin_name);
                    continue;
                }
            };

            let mut proc = match command_builder.spawn() {
                Ok(proc) => proc,
                Err(error) => {
                    log::error!("Failed to spawn plugin command: {}", error);
                    result = Err(BuildError);
                    break;
                }
            };

            let stdout = proc.stdout.take();
            let thread_stdout = thread::spawn(|| match stdout {
                Some(stdout) => {
                    let reader = BufReader::new(stdout);
                    print_lines(reader, "[stdout] ");
                }
                None => {
                    log::error!("Failed to get stdout of plugin command");
                }
            });
            let stderr = proc.stderr.take();
            let thread_stderr = thread::spawn(|| match stderr {
                Some(stderr) => {
                    let reader = BufReader::new(stderr);
                    print_lines(reader, "[stderr] ");
                }
                None => {
                    log::error!("Failed to get stderr of plugin command");
                }
            });

            if let Err(error) = thread_stdout.join() {
                log::error!("Thread related error: {:?}", error);
            }
            if let Err(error) = thread_stderr.join() {
                log::error!("Thread related error: {:?}", error);
            }

            match proc.wait() {
                Ok(status) => {
                    if status.success() {
                        log::info!(r#"Plugin "{}" ran successfully"#, plugin_name);
                    } else {
                        log::error!(r#"Plugin "{}" did not run successfully"#, plugin_name);
                        result = Err(BuildError);
                    }
                }
                Err(error) => {
                    log::error!("Failed to wait for plugin process: {}", error);
                    result = Err(BuildError);
                }
            };
        }
        for unhandled_skip in unhandled_skips {
            log::warn!(
                r#"Plugin named "{}" which is subject to be skipped does not exist"#,
                unhandled_skip
            );
        }

        log::debug!("Creating fingerprint");
        for pack in pack::packs() {
            if !health::source_has_content(self, &pack) {
                continue;
            }
            let path = paths::project::prebuild_of_pack_for_profile(&self.root, pack, profile);
            if let Err(error) = fs::write(path.join(FINGERPRINT), self.id.as_bytes()) {
                log::error!("Failed to create fingerprint: {}", error);
                result = Err(BuildError);
            };
        }

        log::debug!("Zipping add-ons");
        if let Err(error) = zip_create_from_directory(
            &paths::project::build_file_for_profile(&self.root, profile),
            &paths::project::prebuild_for_profile(&self.root, profile),
        ) {
            log::error!("Failed to create zip from prebuild: {}", error);
            result = Err(BuildError);
        }

        if sync {
            // TODO: refactor by breaking down into functions
            match env::var_os("COM_MOJANG") {
                Some(com_mojang) => {
                    log::debug!("Syncing add-ons");
                    let path = PathBuf::from(com_mojang);
                    if !path.is_dir() {
                        log::error!(
                            "The path `{}` of environment variable `COM_MOJANG` is not a directory",
                            path.display()
                        );
                    } else {
                        for pack in pack::packs() {
                            if !health::source_has_content(self, &pack) {
                                continue;
                            }
                            let (source, subdir) = match (pack, profile) {
                                (Pack::Behavior, Profile::Debug) => (
                                    paths::project::prebuild_bp_debug(&self.root),
                                    PathBuf::from("development_behavior_packs"),
                                ),
                                (Pack::Behavior, Profile::Release) => (
                                    paths::project::prebuild_bp_release(&self.root),
                                    PathBuf::from("behavior_packs"),
                                ),
                                (Pack::Resource, Profile::Debug) => (
                                    paths::project::prebuild_rp_debug(&self.root),
                                    PathBuf::from("development_resource_packs"),
                                ),
                                (Pack::Resource, Profile::Release) => (
                                    paths::project::prebuild_rp_release(&self.root),
                                    PathBuf::from("resource_packs"),
                                ),
                                (Pack::Skin, Profile::Debug) => (
                                    paths::project::prebuild_sp_debug(&self.root),
                                    PathBuf::from("development_skin_packs"),
                                ),
                                (Pack::Skin, Profile::Release) => (
                                    paths::project::prebuild_sp_release(&self.root),
                                    PathBuf::from("skin_packs"),
                                ),
                                (Pack::WorldTemplate, Profile::Debug) => (
                                    paths::project::prebuild_wt_debug(&self.root),
                                    PathBuf::from("world_template"),
                                ),
                                (Pack::WorldTemplate, Profile::Release) => (
                                    paths::project::prebuild_wt_release(&self.root),
                                    PathBuf::from("world_template"),
                                ),
                            };
                            let dir = path.join(subdir);
                            log::debug!("Syncing {}", pack);
                            let dest = match find_matching_addon(&dir, &slugified_name, &self.id) {
                                Some(matching_addon) => {
                                    log::debug!(
                                        "Found matching add-on: {}",
                                        matching_addon.display()
                                    );
                                    if let Err(error) = fs::remove_dir_all(&matching_addon) {
                                        log::error!(
                                            "Error while deleting old add-on during sync: {}",
                                            error
                                        );
                                        result = Err(BuildError);
                                    }
                                    if let Err(error) = fs::create_dir(&matching_addon) {
                                        log::error!(
                                            "Error while creating new add-on during sync: {}",
                                            error
                                        );
                                        result = Err(BuildError);
                                    }
                                    matching_addon
                                }
                                None => dir.join(&slugified_name),
                            };

                            let copy_options = fs_extra::dir::CopyOptions::new()
                                .copy_inside(true)
                                .content_only(true);

                            if let Err(error) = fs_extra::dir::copy(&source, &dest, &copy_options) {
                                log::error!(
                                    "Failed to copy directory `{}` to `{}`: {}",
                                    source.display(),
                                    dest.display(),
                                    error
                                );
                            }
                        }
                    }
                }
                None => {
                    log::warn!("Environment variable `COM_MOJANG` is required to be set for syncing; use `--no-sync` or set `sync` in the `[build]` section to `false` to disable synchronization");
                }
            };
        }

        log::debug!("Updating last_build file");
        let last_build_path = paths::project::last_build_for_profile(&self.root, profile);
        if let Err(error) = fs::write(last_build_path, [if result.is_ok() { 1_u8 } else { 0_u8 }]) {
            log::error!("Failed to access last_build file: {}", error);
            result = Err(BuildError);
        };

        log::debug!("Updating build num");
        let last_build_num_path = paths::project::build_num_for_profile(&self.root, profile);
        let num = self
            .build_num_for_profile(profile)
            .map(|num| num + 1)
            .unwrap_or(0);
        let mut last_build_num: Vec<u8> = Vec::new();
        if let Err(error) = last_build_num.write_u32::<BigEndian>(num) {
            log::error!("Failed to write build num: {}", error);
            result = Err(BuildError);
        } else if let Err(error) = fs::write(last_build_num_path, last_build_num) {
            log::error!("Failed to access last_build_num: {}", error);
            result = Err(BuildError);
        }

        self.lock = Some(updated_lock);

        let end_time = SystemTime::now();

        let took = end_time.duration_since(start_time).unwrap();

        match result {
            Ok(_) => {
                let suffix = if took < Duration::from_secs(1) {
                    format!(" {}", unicode_names2_macros::named_char!("Fire"))
                } else {
                    String::new()
                };
                log::info!(
                    "Built project in {}{}",
                    humantime::format_duration(took),
                    suffix
                );
            }
            Err(_) => {
                log::error!(
                    "Built project with errors in {}",
                    humantime::format_duration(took)
                );
            }
        }
        result
    }

    fn all_plugins_are_pure(&self) -> bool {
        self.config.plugins.iter().all(|plugin| plugin.pure)
    }
}

fn print_lines<T>(reader: BufReader<T>, prefix: &str)
where
    T: io::Read,
{
    for maybe_line in reader.lines() {
        match maybe_line {
            Ok(line) => {
                log::info!("{}{}", prefix, line);
            }
            Err(error) => {
                log::error!("Failed to display line: {}", error);
            }
        }
    }
}

/// An invalid project.
#[derive(Debug)]
pub enum InvalidProject {
    /// The project is missing a configuration file (`allay.toml`).
    MissingConfigFile,

    /// The project is missing a project ID file (`.allay/db/project_id.uuid`).
    MissingProjectId,

    /// There was no configuration file while searching the project's root.
    ConfigFileNotFound,

    /// The configuration file is either invalid TOML or does not conform
    /// the configuration specification.
    InvalidConfigFile(toml::de::Error),

    /// The lock file is either invalid TOML or does not conform
    /// the lock specification.
    InvalidLockFile(toml::de::Error),

    /// The format of the project ID is incorrect.
    InvalidProjectId(Vec<u8>),

    /// An I/O related error occured.
    IO(io::Error),
}

impl From<io::Error> for InvalidProject {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl fmt::Display for InvalidProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingConfigFile =>
                    "the project is missing a configuration file".to_string(),
                Self::MissingProjectId => "the project is missing a project id".to_string(),
                Self::ConfigFileNotFound =>
                    "no configuration file was found while searching the project's root".to_string(),
                Self::InvalidConfigFile(error) => error.to_string(),
                Self::InvalidLockFile(error) => error.to_string(),
                Self::InvalidProjectId(value) => format!("inavlid project id: {:?}", value),
                Self::IO(error) => error.to_string(),
            }
        )
    }
}

impl std::error::Error for InvalidProject {}
