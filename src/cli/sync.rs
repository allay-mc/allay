use clap::{ArgMatches, Command};
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

pub fn cmd() -> Command {
    Command::new("sync").about("Update the associated packs in the Minecraft directories")
}

pub fn run(matches: &ArgMatches) -> ExitCode {
    let com_mojang: PathBuf = match env::var_os("COM_MOJANG") {
        Some(var) => PathBuf::from(var),
        None => match location::get() {
            Some(loc) => loc,
            None => return ExitCode::FAILURE,
        },
    };
    todo!("find packs with same id as current project and update them");
}
