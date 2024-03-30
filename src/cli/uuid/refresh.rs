use std::fs;

use crate::paths;
use crate::uuid::Uuids;
use crate::Pack;
use clap::{Arg, ArgMatches, Command};

pub fn cmd() -> Command {
    Command::new("refresh")
        .about("Manually or automatically refresh the project's UUIDs")
        .arg(
            Arg::new("pack")
                .help("Specifies the pack whose UUIDs should be refreshed or all if omitted")
                .required(false)
                .value_parser(clap::value_parser!(Pack)),
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
                .required(false)
                .value_parser(clap::value_parser!(libuuid::Uuid)),
        )
}

pub fn run(matches: &ArgMatches) {
    let packs: Vec<Pack> = matches
        .get_one("pack")
        .map(|p| vec![*p])
        .unwrap_or(Pack::VALUES.to_vec());
    let kinds: Vec<String> = matches
        .get_one("for")
        .map(|k: &String| vec![k.clone()])
        .unwrap_or_else(|| vec!["header".to_string(), "module".to_string()]);
    let uuid: Option<&libuuid::Uuid> = matches.get_one("uuid");

    let path = paths::root().join(paths::uuids());
    let data = fs::read_to_string(path).expect("cannot read UUIDs file");
    let mut uuids: Uuids = toml::from_str(&data).expect("invalid TOML");

    update(&mut uuids, packs, kinds, uuid);
    fs::write(paths::root().join(paths::uuids()), uuids.to_string()).expect("failed to save UUIDs");
}

fn update(uuids: &mut Uuids, packs: Vec<Pack>, kinds: Vec<String>, uuid: Option<&libuuid::Uuid>) {
    for data in vec![
        (&mut uuids.bp, Pack::Behavior),
        (&mut uuids.rp, Pack::Resource),
        (&mut uuids.sp, Pack::Skin),
        (&mut uuids.wt, Pack::WorldTemplate),
    ]
    .into_iter()
    .filter(|i| packs.contains(&i.1))
    {
        if kinds.contains(&&"header".to_string()) {
            data.0.update_header(uuid.copied());
            log::info!("Refreshed header UUID for {}", data.1);
        }
        if kinds.contains(&&"module".to_string()) {
            data.0.update_module(uuid.copied());
            log::info!("Refreshed module UUID for {}", data.1);
        }
    }
}
