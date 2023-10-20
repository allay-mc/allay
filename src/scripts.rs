use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::process::Command;
use std::str;

use anyhow::Context;

use crate::configuration::config::Script;
use crate::environment::Environment;
use crate::paths;
use crate::utils::crate_version;

pub(crate) fn envs(env: &Environment) -> HashMap<String, OsString> {
    let mut vars = HashMap::new();
    vars.insert(
        String::from("ALLAY_BP_PATH"),
        paths::root().join(paths::prebuild_bp()).into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_RP_PATH"),
        paths::root().join(paths::prebuild_rp()).into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_SP_PATH"),
        paths::root().join(paths::prebuild_sp()).into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_WT_PATH"),
        paths::root().join(paths::prebuild_wt()).into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_PREBUILD"),
        paths::root().join(paths::prebuild()).into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_BUILD"),
        paths::root()
            .join(paths::root().join(paths::build()))
            .into_os_string(),
    );
    vars.insert(
        String::from("ALLAY_CONFIG"),
        paths::root().join(paths::config()).into_os_string(),
    );
    vars.insert(String::from("ALLAY_ROOT"), paths::root().into_os_string());
    vars.insert(String::from("ALLAY_VERSION"), crate_version!().into());
    vars.insert(
        String::from("ALLAY_RELEASE"),
        match env.development {
            Some(true) => OsString::from("0"),
            Some(false) => OsString::from("1"),
            None => unreachable!(),
        },
    );
    vars
}

fn run_scripts(env: &Environment, scripts: &Vec<Script>) -> anyhow::Result<()> {
    let mut successful_runs: u32 = 0;
    let base_path = fs::canonicalize(env.config.as_ref().unwrap().scripts.base_path.clone())
        .with_context(|| "cannot make base path absolute maybe because the path does not exist")?;
    for script in scripts {
        let mut args = script.args.clone();
        let prog = script.with.as_ref().unwrap_or(&script.run);
        let kind: &str;
        let name: &str;
        if script.with.is_none() {
            kind = "script";
            name = &script.run;
            args.insert(0, script.run.clone());
        } else {
            kind = "executable";
            name = prog.as_str();
        }

        log::info!("running {} {}", kind, name);
        let output = Command::new(prog)
            .args(args)
            .current_dir(&std::path::Path::new(&base_path))
            .envs(envs(env))
            .output()
            .with_context(|| format!("failed to run {} {}", kind, name))?;
        if output.status.success() {
            successful_runs += 1;
        } else {
            log::error!(
                "{} exited unsuccessfully{}",
                kind,
                match output.status.code() {
                    Some(code) => format!(" (code: {})", code),
                    None => String::from(""),
                }
            )
        }
        if !output.stdout.is_empty() {
            println!("=== Captured stdout of {}", name);
            print!(
                "{}",
                str::from_utf8(&output.stdout).with_context(|| "invalid stdout output")?
            );
            println!("=== End");
        }
        if !output.stderr.is_empty() {
            println!("=== Captured stderr of {}", name);
            print!(
                "{}",
                str::from_utf8(&output.stderr).with_context(|| "invalid stderr output")?
            );
            println!("=== End");
        }
    }
    log::info!("successfully run {} script(s)", successful_runs);
    Ok(())
}

pub(crate) fn run_pre_scripts(env: &Environment) -> anyhow::Result<()> {
    run_scripts(env, &env.config.as_ref().unwrap().scripts.pre)
}

pub(crate) fn run_post_scripts(env: &Environment) -> anyhow::Result<()> {
    run_scripts(env, &env.config.as_ref().unwrap().scripts.post)
}
