use std::fs::File;

use clap::{Arg, ArgMatches, Command};
use uuid::Uuid;

use crate::paths;
use crate::{addon::AddonType, build::uuidgen};

pub(crate) fn cmd() -> Command {
    Command::new("refresh")
        .about("Manually or automatically refresh the UUIDs of this project")
        .arg(
            Arg::new("pack")
                .help("Specifies the pack whose UUIDs should be refreshed or all if omitted")
                .required(false)
                .value_parser(clap::value_parser!(AddonType)),
        )
        .arg(
            Arg::new("for")
                .help("Whether the header or module UUID should be changed")
                .required(false)
                .value_parser(["header", "module"])
                .ignore_case(true),
        )
        .arg(
            Arg::new("uuid")
                .help("Manually specify a UUID")
                .required(false),
        )
}

pub(crate) fn run(matches: &ArgMatches) {
    let pack: Option<&AddonType> = matches.get_one("pack");
    let kind: Option<&String> = matches.get_one("for");
    let uuid: Option<&String> = matches.get_one("uuid");
    let mut table = uuidgen::read_uuids().expect("cannot read UUIDs");
    match pack {
        Some(AddonType::BehaviorPack) => match kind.map(|v| v.as_str()) {
            Some("header") => {
                uuidgen::update_bp_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            Some("module") => {
                uuidgen::update_bp_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            None => {
                uuidgen::update_bp_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
                uuidgen::update_bp_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            _ => unreachable!(),
        },
        Some(AddonType::ResourcePack) => match kind.map(|v| v.as_str()) {
            Some("header") => {
                uuidgen::update_rp_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            Some("module") => {
                uuidgen::update_rp_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            None => {
                uuidgen::update_rp_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
                uuidgen::update_rp_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            _ => unreachable!(),
        },
        Some(AddonType::SkinPack) => match kind.map(|v| v.as_str()) {
            Some("header") => {
                uuidgen::update_sp_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            Some("module") => {
                uuidgen::update_sp_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            None => {
                uuidgen::update_sp_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
                uuidgen::update_sp_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            _ => unreachable!(),
        },
        Some(AddonType::WorldTemplate) => match kind.map(|v| v.as_str()) {
            Some("header") => {
                uuidgen::update_wt_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            Some("module") => {
                uuidgen::update_wt_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            None => {
                uuidgen::update_wt_header(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
                uuidgen::update_wt_module(
                    &mut table,
                    uuid.cloned().unwrap_or_else(|| Uuid::new_v4().to_string()),
                );
            }
            _ => unreachable!(),
        },
        None => {
            uuidgen::update_bp_header(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_bp_module(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_rp_header(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_rp_module(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_sp_header(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_sp_module(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_wt_header(&mut table, Uuid::new_v4().to_string());
            uuidgen::update_wt_module(&mut table, Uuid::new_v4().to_string());
        }
    };

    uuidgen::save_uuids(&table).expect("cannot save UUIDs")
}
