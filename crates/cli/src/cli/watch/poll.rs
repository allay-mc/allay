use std::{
    collections::HashMap,
    fs::{self, Metadata},
    path::Path,
};

#[derive(Debug, Default)]
pub struct Watcher<'a> {
    watch_paths: HashMap<&'a Path, Option<Metadata>>,
    ignore_paths: Vec<&'a Path>,
}

impl<'a> Watcher<'a> {
    pub fn new(watch_paths: Vec<&'a Path>) -> Self {
        let mut map = HashMap::new();
        for path in watch_paths {
            map.insert(path, None);
        }
        Self {
            watch_paths: map,
            ignore_paths: Vec::new(),
        }
    }

    /// Adds a path to the watch list.
    pub fn watch(&mut self, watch_path: &'a Path) -> &mut Self {
        self.watch_paths.insert(watch_path, None);
        self
    }

    /// Adds a path to ignore for watching changes.
    pub fn ignore(&mut self, ignore_path: &'a Path) -> &mut Self {
        self.ignore_paths.push(ignore_path);
        self
    }

    /// Returns the paths that changed since last scan.
    pub fn changes(&mut self) -> Vec<&'a Path> {
        let mut changed: Vec<&'a Path> = Vec::new();
        // TODO: check for newly created paths
        for (path, metadata) in self.watch_paths.iter_mut() {
            for ignore_path in &self.ignore_paths {
                if ignore_path
                    .canonicalize()
                    .is_ok_and(|p| Some(p) == path.canonicalize().ok())
                {
                    continue;
                }
            }

            let Ok(new_metadata) = fs::metadata(path) else {
                continue;
            };
            if let Some(old_metadata) = metadata {
                if metadata_differs(old_metadata, &new_metadata) {
                    changed.push(path);
                    *metadata = Some(new_metadata.clone());
                }
            }
            if new_metadata.is_dir() {
                // TODO: recursively check for changes
            }
        }
        changed
    }
}

fn metadata_differs(md1: &Metadata, md2: &Metadata) -> bool {
    md1.file_type() != md2.file_type()
        || md1.len() != md2.len()
        || md1.modified().ok() == md2.modified().ok()
}
