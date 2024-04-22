use super::build;
use super::prelude::*;
use crate::paths;
use crate::Project;
use clap::{ArgMatches, Command};
use libuuid::Uuid;
use std::fs;
use std::str::FromStr;
use std::{env, path::PathBuf, process::ExitCode};

const DEV_BP: &'static str = "development_behavior_packs";
const DEV_RP: &'static str = "development_resource_packs";
const DEV_SP: &'static str = "development_skin_packs";

mod location {
    use crate::diagnostic;
    use std::{env, path::PathBuf};

    fn android_termux() -> Option<PathBuf> {
        let path = PathBuf::from(env::var_os("HOME").expect("HOME environment variable not set"))
            .join("storage/shared/Android/data/com.mojang.minecraftpe/files/games/com.mojang");
        if path.exists() {
            Some(path)
        } else {
            log::error!("{}", diagnostic::Notification::ComMojangNotFoundAndroid);
            None
        }
    }

    fn windows() -> Option<PathBuf> {
        let path = PathBuf::from(
            env::var_os("LOCALAPPDATA").expect("LOCALAPPDATA environment variable not set"),
        )
        .join("Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang");
        if path.exists() {
            Some(path)
        } else {
            log::error!("{}", diagnostic::Notification::ComMojangWindows);
            None
        }
    }

    pub fn get() -> Option<PathBuf> {
        if cfg!(android) {
            android_termux()
        } else if cfg!(ios) {
            log::error!("iOS devices are not supported for syncing yet");
            None
        } else if cfg!(windows) {
            windows()
        } else {
            log::error!("Your OS is not supported for syncing");
            None
        }
    }
}

fn update(dev_dir: &PathBuf, projet_id: &Uuid) -> bool {
    for pack in dev_dir.read_dir().expect("failed to read dir") {
        match pack {
            Ok(pack) => {
                let path = pack.path();
                if !path.is_dir() {
                    continue;
                }
                let fingerprint = path.join(paths::FINGERPRINT);
                match fs::read_to_string(fingerprint) {
                    Ok(value) => {
                        return Uuid::from_str(&value).is_ok_and(|value| &value == projet_id);
                    }
                    Err(e) => log::error!("failed to read fingerprint file: {}", e),
                }
            }
            Err(e) => log::error!("failed to read entry of pack direcrory: {}", e),
        };
    }
    false
}

pub fn cmd() -> Command {
    Command::new("sync")
        .about("Update the associated packs in the Minecraft directories")
        .arg_build_opts()
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    build::run(matches);
    let id = Project::current().unwrap().id;

    let com_mojang: PathBuf = match env::var_os("COM_MOJANG") {
        Some(var) => PathBuf::from(var),
        None => match location::get() {
            Some(loc) => loc,
            None => return ExitCode::FAILURE,
        },
    };

    let mut updated_any = false;
    for entry in com_mojang
        .read_dir()
        .expect("cannot read com_mojang directory")
    {
        match entry {
            Ok(entry) => {
                for dir in [DEV_BP, DEV_RP, DEV_SP] {
                    let p = entry.path().join(dir);
                    if p.is_dir() {
                        let updated = update(&p, &id);
                        if updated {
                            updated_any = true;
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read entry of com_mojang directory: {}", e);
            }
        }
    }

    if updated_any {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
