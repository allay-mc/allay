use std::collections::HashMap;

use crate::addon::AddonType;
use crate::configuration::manifest::{Dependency, Header, Manifest, Metadata, Module, ModuleType};
use crate::environment::Environment;
use crate::utils::{crate_version, version_as_array};

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
            min_engine_version: env
                .config
                .as_ref()
                .unwrap()
                .project
                .min_engine_version
                .clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::bp_header(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: env.config.as_ref().unwrap().pack.bp.kind,
            uuid: uuidgen::bp_module(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        }]),
        dependencies: Some(if AddonType::BehaviorPack.exists() {
            let mut deps = env.config.as_ref().unwrap().pack.bp.dependencies.clone();
            deps.push(Dependency {
                uuid: uuidgen::rp_header(&env.uuids.as_ref().unwrap())
                    .expect("RP header UUID not present"),
                version: version_as_array(&env.config.as_ref().unwrap().project.version),
            });
            deps
        } else {
            env.config.as_ref().unwrap().pack.bp.dependencies.clone()
        }),
        capabilities: Some(env.config.as_ref().unwrap().pack.capabilities.clone()),
        metadata: Some(Metadata {
            authors: env.config.as_ref().unwrap().project.authors.clone(),
            license: env.config.as_ref().unwrap().project.license.clone(),
            url: env.config.as_ref().unwrap().project.url.clone(),
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
            min_engine_version: env
                .config
                .as_ref()
                .unwrap()
                .project
                .min_engine_version
                .clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::rp_header(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: ModuleType::Resources,
            uuid: uuidgen::rp_module(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        }]),
        dependencies: Some(if AddonType::ResourcePack.exists() {
            let mut deps = env.config.as_ref().unwrap().pack.bp.dependencies.clone();
            deps.push(Dependency {
                uuid: uuidgen::bp_header(&env.uuids.as_ref().unwrap())
                    .expect("BP header UUID not present"),
                version: version_as_array(&env.config.as_ref().unwrap().project.version),
            });
            deps
        } else {
            env.config.as_ref().unwrap().pack.bp.dependencies.clone()
        }),
        capabilities: Some(env.config.as_ref().unwrap().pack.capabilities.clone()),
        metadata: Some(Metadata {
            authors: env.config.as_ref().unwrap().project.authors.clone(),
            license: env.config.as_ref().unwrap().project.license.clone(),
            generated_with,
            url: env.config.as_ref().unwrap().project.url.clone(),
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
            min_engine_version: env
                .config
                .as_ref()
                .unwrap()
                .project
                .min_engine_version
                .clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::sp_header(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: ModuleType::SkinPack,
            uuid: uuidgen::sp_module(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        }]),
        dependencies: None,
        capabilities: None,
        metadata: Some(Metadata {
            authors: env.config.as_ref().unwrap().project.authors.clone(),
            license: env.config.as_ref().unwrap().project.license.clone(),
            generated_with,
            url: env.config.as_ref().unwrap().project.url.clone(),
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
            allow_random_seed: env.config.as_ref().unwrap().pack.wt.allow_random_seed,
            base_game_version: Some(version_as_array(
                &env.config
                    .as_ref()
                    .unwrap()
                    .pack
                    .wt
                    .base_game_version
                    .clone()
                    .expect("missing base game version for world template")
                    .as_str(),
            )),
            description: String::from("pack.description"),
            lock_template_options: env.config.as_ref().unwrap().pack.wt.lock_template_options,
            min_engine_version: env
                .config
                .as_ref()
                .unwrap()
                .project
                .min_engine_version
                .clone(),
            name: String::from("pack.name"),
            uuid: uuidgen::wt_header(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        },
        modules: Some(vec![Module {
            description: String::from("pack.description"),
            kind: ModuleType::WorldTemplate,
            uuid: uuidgen::wt_module(&env.uuids.as_ref().unwrap()).unwrap(),
            version: version_as_array(env.config.as_ref().unwrap().project.version.as_str()),
        }]),
        dependencies: None,
        capabilities: None,
        metadata: Some(Metadata {
            authors: env.config.as_ref().unwrap().project.authors.clone(),
            license: env.config.as_ref().unwrap().project.license.clone(),
            generated_with,
            url: env.config.as_ref().unwrap().project.url.clone(),
        }),
    }
}
