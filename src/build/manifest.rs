use std::collections::HashMap;

use crate::configuration::manifest::{Dependency, Header, Manifest, Metadata, Module, ModuleType};
use crate::environment::Environment;
use crate::utils::{self, crate_version, version_as_array};

use super::uuidgen;

pub(crate) fn behavior_pack(env: &Environment) -> Manifest {
    let mut generator: HashMap<String, Vec<String>> = HashMap::new();
    generator.insert(String::from("allay"), vec![crate_version!().to_string()]);
    let generated_with = Some(generator);

    Manifest {
        format_version: 1,
        header: Header {
            allow_random_seed: None,
            base_game_version: None,
            description: String::from("pack.description"),
            lock_template_options: None,
            min_engine_version: env.config.project.min_engine_version.clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::bp_header(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: env.config.pack.bp.kind,
            uuid: uuidgen::bp_module(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        }]),
        dependencies: Some(if utils::has_rp() {
            let mut deps = env.config.pack.bp.dependencies.clone();
            deps.push(Dependency {
                uuid: uuidgen::rp_header(&env.uuids).expect("RP header UUID not present"),
                version: version_as_array(&env.config.project.version),
            });
            deps
        } else {
            env.config.pack.bp.dependencies.clone()
        }),
        capabilities: Some(env.config.pack.capabilities.clone()),
        metadata: Some(Metadata {
            authors: env.config.project.authors.clone(),
            license: env.config.project.license.clone(),
            url: env.config.project.url.clone(),
            generated_with,
        }),
    }
}

pub(crate) fn resource_pack(env: &Environment) -> Manifest {
    let mut generator: HashMap<String, Vec<String>> = HashMap::new();
    generator.insert(String::from("allay"), vec![crate_version!().to_string()]);
    let generated_with = Some(generator);

    Manifest {
        format_version: 1,
        header: Header {
            allow_random_seed: None,
            base_game_version: None,
            description: String::from("pack.description"),
            lock_template_options: None,
            min_engine_version: env.config.project.min_engine_version.clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::rp_header(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: ModuleType::Resources,
            uuid: uuidgen::rp_module(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        }]),
        dependencies: Some(if utils::has_bp() {
            let mut deps = env.config.pack.bp.dependencies.clone();
            deps.push(Dependency {
                uuid: uuidgen::bp_header(&env.uuids).expect("BP header UUID not present"),
                version: version_as_array(&env.config.project.version),
            });
            deps
        } else {
            env.config.pack.bp.dependencies.clone()
        }),
        capabilities: Some(env.config.pack.capabilities.clone()),
        metadata: Some(Metadata {
            authors: env.config.project.authors.clone(),
            license: env.config.project.license.clone(),
            generated_with,
            url: env.config.project.url.clone(),
        }),
    }
}

pub(crate) fn skin_pack(env: &Environment) -> Manifest {
    let mut generator: HashMap<String, Vec<String>> = HashMap::new();
    generator.insert(String::from("allay"), vec![crate_version!().to_string()]);
    let generated_with = Some(generator);

    Manifest {
        format_version: 2,
        header: Header {
            allow_random_seed: None,
            base_game_version: None,
            description: String::from("pack.description"),
            lock_template_options: None,
            min_engine_version: env.config.project.min_engine_version.clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::sp_header(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: ModuleType::SkinPack,
            uuid: uuidgen::sp_module(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        }]),
        dependencies: None,
        capabilities: None,
        metadata: Some(Metadata {
            authors: env.config.project.authors.clone(),
            license: env.config.project.license.clone(),
            generated_with,
            url: env.config.project.url.clone(),
        }),
    }
}

pub(crate) fn world_template(env: &Environment) -> Manifest {
    let mut generator: HashMap<String, Vec<String>> = HashMap::new();
    generator.insert(String::from("allay"), vec![crate_version!().to_string()]);
    let generated_with = Some(generator);

    Manifest {
        format_version: 2,
        header: Header {
            allow_random_seed: env.config.pack.wt.allow_random_seed,
            base_game_version: Some(version_as_array(
                &env.config
                    .pack
                    .wt
                    .base_game_version
                    .clone()
                    .expect("missing base game version for world template")
                    .as_str(),
            )),
            description: String::from("pack.description"),
            lock_template_options: env.config.pack.wt.lock_template_options,
            min_engine_version: env.config.project.min_engine_version.clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::wt_header(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: ModuleType::WorldTemplate,
            uuid: uuidgen::wt_module(&env.uuids).unwrap(),
            version: version_as_array(env.config.project.version.as_str()),
        }]),
        dependencies: None,
        capabilities: None,
        metadata: Some(Metadata {
            authors: env.config.project.authors.clone(),
            license: env.config.project.license.clone(),
            generated_with,
            url: env.config.project.url.clone(),
        }),
    }
}
