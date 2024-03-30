use super::prelude::*;
use crate::{paths, Project};
use clap::{ArgMatches, Command};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(1);

pub fn cmd() -> Command {
    Command::new("watch")
        .visible_alias("w")
        .about("Watch the source files and rebuilds the add-ons on changes")
        .arg_build_opts()
}

pub fn run(_matches: &ArgMatches) -> ExitCode {
    let project = Project::current().unwrap();
    trigger_on_change(&project, |paths, root| {
        log::info!("Files changed: {:?}; Building project...", paths);
        // TODO: debug/release mode
        if let Err(e) = Project::from_root(&root.to_path_buf())
            .expect("valid project root path")
            .build()
        {
            log::error!("Unable to build project: {}", e);
        };
    });
    ExitCode::SUCCESS
}

fn trigger_on_change<F>(project: &Project, closure: F)
where
    F: Fn(Vec<PathBuf>, &Path),
{
    use notify::RecursiveMode::*;
    let (tx, rx) = channel();

    let mut debouncer = match notify_debouncer_mini::new_debouncer(TIMEOUT, tx) {
        Ok(d) => d,
        Err(e) => {
            log::error!("Error while trying to watch the files: {}", e);
            std::process::exit(1)
        }
    };
    let watcher = debouncer.watcher();

    let src = paths::src();
    if let Err(e) = watcher.watch(&src, Recursive) {
        log::error!("Error while watching {:?}: {}", &src, e);
        std::process::exit(1);
    };

    let _ = watcher.watch(&paths::root().join(paths::config()), Recursive);

    for dir in &project.config.build.extra_watch_dirs {
        let path = paths::root().join(dir);
        let canonical_path = path.canonicalize().unwrap_or_else(|e| {
            log::error!("Error while watching extra directory {:?}: {}", path, e);
            std::process::exit(1);
        });

        if let Err(e) = watcher.watch(&canonical_path, Recursive) {
            log::error!(
                "Error while watching extra directory {:?}: {:?}",
                canonical_path,
                e
            );
            std::process::exit(1);
        }
    }

    log::info!("Listening for changes ...");

    loop {
        let first_event = rx.recv().unwrap();
        thread::sleep(Duration::from_millis(50));
        let other_events = rx.try_iter();

        let all_events = std::iter::once(first_event).chain(other_events);

        let paths: Vec<_> = all_events
            .filter_map(|event| match event {
                Ok(events) => Some(events),
                Err(error) => {
                    log::warn!("Error while watching for changes: {error}");
                    None
                }
            })
            .flatten()
            .map(|event| event.path)
            .collect();

        // If we are watching files outside the current repository (via extra-watch-dirs), then they are definitionally
        // ignored by gitignore. So we handle this case by including such files into the watched paths list.
        let any_external_paths = paths
            .iter()
            .filter(|p| !p.starts_with(&paths::root()))
            .cloned();
        let mut paths = remove_ignored_files(&paths::root(), &paths[..]);
        paths.extend(any_external_paths);

        if !paths.is_empty() {
            closure(paths, &paths::root());
        }
    }
}

fn remove_ignored_files(_root: &Path, paths: &[PathBuf]) -> Vec<PathBuf> {
    // TODO: gitignore, see: https://github.com/rust-lang/mdBook/blob/master/src/cmd/watch.rs#L60
    if paths.is_empty() {
        Vec::new()
    } else {
        paths.iter().map(|path| path.to_path_buf()).collect()
    }
}
