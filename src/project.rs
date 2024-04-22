use crate::health::has_content;
use crate::localization::{
    generate_language_json, update_language_files, Localized, OptionallyLocalized,
};
use crate::plugin::{ExecutablePlugin, Plugin};
use crate::Config;
use crate::Error;
use crate::Manifest;
use crate::Pack;
use crate::{diagnostic, uuid};
use crate::{filter, paths, scaffolding, Health};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str;
use zip_extensions::write::zip_create_from_directory;

#[derive(Clone, Copy, Debug)]
pub struct ProjectInitConfig {
    /// Whether to generate a `.gitignore` file.
    pub with_gitignore: bool,

    #[cfg(feature = "git")]
    /// Whether to initialite a new git repository.
    pub init_git: bool,
}

#[derive(Clone)]
pub struct Project {
    /// The configuration for the project.
    pub config: Config,

    /// The UUIDs for the packs.
    pub uuids: uuid::Uuids,

    /// The unique ID of the project.
    pub id: libuuid::Uuid,
}

impl Project {
    pub fn new(dir: &PathBuf, force: bool, config: ProjectInitConfig) -> Result<Self, io::Error> {
        let empty = dir.read_dir()?.count().eq(&0);
        if !empty && !force {
            return Err(io::Error::new(
                // TODO: change to `io::ErrorKind::DirectoryNotEmpty` when https://github.com/rust-lang/rust/issues/86442
                //       is stabalized.
                io::ErrorKind::Other,
                "cannot initialize non-empty directory",
            ));
        }

        let id = libuuid::Uuid::new_v4();
        let uuids = uuid::Uuids::default();

        fs::create_dir(dir.join(paths::internal()))?;
        fs::create_dir(dir.join(paths::src()))?;
        fs::create_dir(dir.join(paths::src_bp()))?;
        fs::create_dir(dir.join(paths::src_rp()))?;
        fs::create_dir(dir.join(paths::src_sp()))?;
        fs::create_dir(dir.join(paths::src_wt()))?;

        fs::write(dir.join(paths::config()), scaffolding::CONFIG)?;
        fs::write(dir.join(paths::pack_icon()), scaffolding::PACK_ICON)?;
        fs::write(dir.join(paths::uuids()), uuids.to_string())?;
        fs::write(dir.join(paths::version()), clap::crate_version!())?;
        fs::write(dir.join(paths::project_id()), id.to_string())?;
        #[cfg(feature = "git")]
        {
            match git2::Repository::init(&dir) {
                Ok(_repo) => {}
                Err(e) => log::error!("Failed to initialize git repository: {}", e),
            };
        };
        if config.with_gitignore {
            fs::write(dir.join(paths::gitignore()), scaffolding::GITIGNORE)?;
        }

        Ok(Self {
            config: Config::from_str(
                str::from_utf8(scaffolding::CONFIG)
                    .expect("Config template is not UTF-8; please report this error"),
            )
            .expect("Config template is invalid; please report this error"),
            uuids,
            id,
        })
    }

    pub fn from_root(root_dir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let cfg = fs::read_to_string(root_dir.join(paths::config()))?;
        Ok(Self {
            config: Config::from_str(&cfg)?,
            uuids: uuid::Uuids::from_str(&fs::read_to_string(root_dir.join(paths::uuids()))?)?,
            id: libuuid::Uuid::parse_str(&fs::read_to_string(root_dir.join(paths::project_id()))?)?,
        })
    }

    /// Returns the project by accessing the user's current working directory.
    pub fn current() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_root(&paths::try_root().ok_or(Error::NotInAProject)?)
    }

    pub fn build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let health = Health {
            root: paths::root(),
            fix: false,
        };
        if !health.check_all_except_uuids() {
            return Err(Box::new(Error::InvalidProjectSetup));
        };

        let prebuild: PathBuf = env::temp_dir().join(format!("allay-{}", self.id));
        if prebuild.exists() {
            log::debug!("Delete old prebuild directory in {}", &prebuild.display());
            fs::remove_dir_all(&prebuild)?;
        }
        log::debug!("Create prebuild directory in {}", &prebuild.display());
        fs::create_dir_all(&prebuild)?;

        let mut packs: Vec<(Pack, PathBuf)> = Vec::new();
        let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);
        for pack in Pack::VALUES {
            if !pack.exists() {
                continue;
            }

            log::debug!("Generating UUIDs for {} if neccessary", pack);
            match pack {
                Pack::Behavior => self.uuids.bp.update_header(None).update_module(None),
                Pack::Resource => self.uuids.rp.update_header(None).update_module(None),
                Pack::Skin => self.uuids.sp.update_header(None).update_module(None),
                Pack::WorldTemplate => self.uuids.wt.update_header(None).update_module(None),
            };

            let dest = prebuild.join(pack.short_name());
            log::debug!("Copy {} source to {}", pack, dest.display());
            fs_extra::dir::copy(
                pack.path_src().ok_or(Error::NotInAProject)?,
                &dest,
                &copy_options,
            )?;

            log::debug!("Adding fingerprint");
            if let Err(e) = fs::write(&dest.join(paths::FINGERPRINT), self.id) {
                log::error!("Failed to add fingerprint: {}", e);
            };

            let generate_manifest = !match pack {
                Pack::Behavior => self.config.bp.custom_manifest,
                Pack::Resource => self.config.rp.custom_manifest,
                Pack::Skin => self.config.sp.custom_manifest,
                Pack::WorldTemplate => self.config.wt.custom_manifest,
            };
            if generate_manifest {
                log::debug!("Generating manifests");
                let mf = Manifest::build(pack, self.clone())?;
                let json = if self.config.debug {
                    serde_json::to_string_pretty(&mf)?
                } else {
                    serde_json::to_string(&mf)?
                };
                let p = dest.join("manifest.json");
                if p.try_exists().unwrap_or(false) && p.is_file() {
                    log::warn!("{}", diagnostic::Notification::RedundantManifest);
                };
                fs::write(&p, json)?;
            }

            let copy_pack_icon = !match pack {
                Pack::Behavior => self.config.bp.custom_pack_icon,
                Pack::Resource => self.config.rp.custom_pack_icon,
                Pack::Skin => self.config.sp.custom_pack_icon,
                Pack::WorldTemplate => false,
            };
            if copy_pack_icon {
                log::debug!("Copying pack icon");
                let p = dest.join("pack_icon.png");
                if p.try_exists().unwrap_or(false) && p.is_file() {
                    log::warn!("{}", diagnostic::Notification::RedundantPackIcon);
                };
                fs::copy(paths::pack_icon(), dest.join("pack_icon.png"))?;
            }

            log::debug!("Generating/extending language files");
            {
                let mut groups = self.config.localization.groups.clone();
                let name: Localized<String> = match &self.config.project.name {
                    OptionallyLocalized::Localized(m) => m.clone(),
                    OptionallyLocalized::Unlocalized(s) => {
                        let mut map = HashMap::new();
                        map.insert(
                            self.config.localization.primary_language.clone(),
                            s.to_string(),
                        );
                        map
                    }
                };
                for l in name.keys() {
                    groups.with_language(l.clone());
                }

                let desc: Localized<String> = match &self.config.project.description {
                    OptionallyLocalized::Localized(m) => m.clone(),
                    OptionallyLocalized::Unlocalized(s) => {
                        let mut map = HashMap::new();
                        map.insert(
                            self.config.localization.primary_language.clone(),
                            s.to_string(),
                        );
                        map
                    }
                };
                for l in desc.keys() {
                    groups.with_language(l.clone());
                }

                let mut translations: HashMap<String, Localized<String>> = HashMap::new();
                translations.insert("pack.name".to_string(), name);
                translations.insert("pack.description".to_string(), desc);

                let texts_dir = dest.join("texts");
                {
                    let res = fs::create_dir(&texts_dir);
                    if res
                        .as_ref()
                        .is_err_and(|e| e.kind() != io::ErrorKind::AlreadyExists)
                    {
                        res?;
                    };
                };
                match update_language_files(
                    &texts_dir,
                    &groups,
                    &self.config.localization.primary_language,
                    translations,
                ) {
                    Ok(_) => {}
                    Err(e) => log::error!("Error while appending language files: {}", e),
                };
                match generate_language_json(&texts_dir) {
                    Ok(_) => {}
                    Err(e) => log::error!("Error while generating languages.json: {}", e),
                };
            };

            packs.push((pack, dest));
        }

        if packs.is_empty() {
            log::warn!("{}", diagnostic::Notification::EmptyAddOn);
            return Ok(());
        }

        log::debug!("Run plugins");
        {
            for plugin in &self.config.plugin {
                if let Some(when) = &plugin.when {
                    match filter::evaluate(when) {
                        Ok(false) => {
                            log::info!(
                                "skipping running plugin {} because filter evaluated false",
                                plugin.name.as_ref().unwrap_or(&"<unnamed>".to_string()) // TODO: change `<unnamed>`
                            );
                            continue;
                        }
                        Ok(true) => {}
                        Err(e) => {
                            log::error!("Filter error\n{}", e);
                            return Err(Box::new(e));
                        }
                    }
                };
                let name = plugin
                    .name
                    .clone()
                    .unwrap_or("<unnamed plugin>".to_string());
                let plugin = ExecutablePlugin::from(plugin);
                let mut envs = Vec::new();
                envs.push((
                    "ALLAY_DEBUG",
                    if self.config.debug {
                        "1".into()
                    } else {
                        "0".into()
                    },
                ));
                envs.push(("ALLAY_PREBUILD", prebuild.clone().into_os_string()));
                envs.push(("ALLAY_PROJECT_ROOT", paths::root().into_os_string()));
                envs.push(("ALLAY_VERSION", clap::crate_version!().into()));
                envs.extend(
                    self.config
                        .env
                        .iter()
                        .map(|(key, value)| (key.as_str(), value.into())),
                );
                let result = plugin.run(envs);
                match result {
                    Ok(res) => log::info!("[{}] {}", name, String::from_utf8_lossy(res.as_slice())),
                    Err(e) => {
                        log::error!("Failed to run plugin {}: {}", name, e);
                        if plugin.panic {
                            return Err(e);
                        }
                    }
                };
            }
        }

        {
            let rp = prebuild.join("RP");
            let bp = prebuild.join("BP");
            let wt = prebuild.join("WT");

            let has_rp = has_content(&Pack::Resource.path_src().unwrap());
            let has_bp = has_content(&Pack::Behavior.path_src().unwrap());
            let has_wt = has_content(&Pack::WorldTemplate.path_src().unwrap());

            if has_wt && !self.config.wt.exclude_bp && has_bp {
                log::debug!("Copying behavior pack to world template");

                let dest = wt.join("behavior_packs");
                // TODO: rename `RP`/`BP` to a less generic name (?)
                let copy = || {
                    if let Err(e) = fs_extra::dir::copy(bp, &dest, &copy_options) {
                        log::error!("Failed to copy behavior pack to world template: {}", e);
                    };
                };
                if dest.exists() {
                    copy()
                } else {
                    match fs::create_dir(&dest) {
                        Ok(_) => copy(),
                        Err(e) => {
                            log::error!(
                                "Failed to create behavior pack directory in world template: {}",
                                e
                            );
                        }
                    };
                }
            }

            if has_wt && !self.config.wt.exclude_rp && has_rp {
                log::debug!("Copying resource pack to world template");

                let dest = wt.join("resource_packs");
                let copy = || {
                    if let Err(e) = fs_extra::dir::copy(rp, &dest, &copy_options) {
                        log::error!("Failed to copy resource pack to world template: {}", e);
                    };
                };
                if dest.exists() {
                    copy();
                } else {
                    match fs::create_dir(&dest) {
                        Ok(_) => copy(),
                        Err(e) => {
                            log::error!(
                                "Failed to create resource pack directory in world template: {}",
                                e
                            );
                        }
                    };
                }
            }
        }

        log::debug!("Zipping add-ons");
        {
            let mut bundles: Vec<PathBuf> = Vec::new();
            for (pack, path) in packs {
                let bundle = prebuild.join(format!(
                    "{}.{}",
                    pack.short_name(),
                    pack.bundle_file_extension()
                ));
                // NOTE: zipping seems to be optional
                zip_create_from_directory(&bundle, &path)?;
                bundles.push(bundle);
            }

            // remove all entries in prebuild excpect bundles
            for entry in prebuild.read_dir()? {
                let entry = entry?.path();
                if !bundles.contains(&entry) {
                    if entry.is_file() {
                        fs::remove_file(entry)?;
                    } else {
                        fs::remove_dir_all(entry)?;
                    }
                }
            }
        }

        log::debug!("Creating build file");
        {
            let build_file = paths::try_root()
                .ok_or(Error::NotInAProject)?
                .join(paths::build());
            zip_create_from_directory(&build_file, &prebuild)?; // TODO: change prebuild to bundled files
        }

        Ok(())
    }
}
