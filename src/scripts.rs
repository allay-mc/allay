use std::collections::HashMap;
use std::ffi::OsString;
use std::process::Command;
use std::{fs, str};

use crate::configuration::config::Script;
use crate::environment::Environment;
use crate::paths;
use crate::utils::crate_version;

pub(crate) fn envs(env: &Environment) -> HashMap<String, OsString> {
    let mut vars = HashMap::new();
    vars.insert(
        String::from("ALLAY_BP_PATH"),
        paths::prebuild_bp().into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_RP_PATH"),
        paths::prebuild_rp().into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_SP_PATH"),
        paths::prebuild_sp().into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_WT_PATH"),
        paths::prebuild_wt().into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_PREBUILD"),
        paths::prebuild().into_os_string(),
    );
    vars.insert(String::from("ALLAY_BUILD"), paths::build().into_os_string());
    vars.insert(
        String::from("ALLAY_CONFIG"),
        paths::config().into_os_string(),
    );
    vars.insert(String::from("ALLAY_ROOT"), paths::root().into_os_string());
    vars.insert(String::from("ALLAY_VERSION"), crate_version!().into());
    vars.insert(
        String::from("ALLAY_RELEASE"),
        if env.development {
            OsString::from("0")
        } else {
            OsString::from("1")
        },
    );
    vars
}

fn run_scripts(env: &Environment, scripts: &Vec<Script>) -> Result<(), String> {
    let base_path = fs::canonicalize(env.config.scripts.base_path.clone())
        .map_err(|_| String::from("cannot make base path absolute"))?;
    for script in scripts {
        let path = script.run.clone();
        log::info!("running script {}", path);
        let output = Command::new(script.with.clone())
            .arg(&path) // TODO: ensure valid path
            .args(script.args.clone())
            .current_dir(&std::path::Path::new(&base_path))
            .envs(envs(env))
            .output()
            .map_err(|_| format!("failed to run script {}", path))?;
        log::debug!("script exited with status {}", output.status);
        if !output.stdout.is_empty() {
            println!("=== Captured stdout of {}", path);
            print!(
                "{}",
                str::from_utf8(&output.stdout).expect("invalid stdout output")
            );
            println!("=== End");
        }
        if !output.stderr.is_empty() {
            println!("=== Captured stderr of {}", path);
            print!(
                "{}",
                str::from_utf8(&output.stderr).expect("invalid stderr output")
            );
            println!("=== End");
        }
    }
    Ok(())
}

pub(crate) fn run_pre_scripts(env: &Environment) -> Result<(), String> {
    run_scripts(env, &env.config.scripts.pre)
}

pub(crate) fn run_post_scripts(env: &Environment) -> Result<(), String> {
    run_scripts(env, &env.config.scripts.post)
}
