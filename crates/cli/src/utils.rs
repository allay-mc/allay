pub(crate) mod build {
    use allay::{
        project::{BuildError, Profile},
        BuildContext, Project,
    };

    /// Wrapper around building projects which does extra validations.
    pub(crate) fn build_project(
        project: &mut Project,
        context: &BuildContext,
    ) -> Result<(), BuildError> {
        project.build(context)
    }

    pub(crate) fn build_context_from_args_and_config(
        matches: &clap::ArgMatches,
        config: &allay::Config,
    ) -> allay::BuildContext {
        let primary_profile: Option<Profile> = if matches.get_flag("build-release") {
            Some(Profile::Release)
        } else if matches.get_flag("build-debug") {
            Some(Profile::Debug)
        } else {
            None
        };

        let sync: Option<bool> = if matches.get_flag("build-enable-sync") {
            Some(true)
        } else if matches.get_flag("build-disable-sync") {
            Some(false)
        } else {
            None
        };

        let force_rebuild = matches.get_flag("build-force");
        let mut build_context = allay::BuildContext::default();
        build_context.with_config(config).with_force(force_rebuild);
        if let Some(profile) = primary_profile {
            build_context.with_profile(profile);
        }
        if let Some(sync) = sync {
            build_context.with_sync(sync);
        }
        for plugin_name in matches
            .get_many::<String>("skip-plugins")
            .unwrap_or_default()
        {
            build_context.skip_plugin(plugin_name.to_string());
        }

        build_context
    }
}

#[cfg(feature = "git")]
pub(crate) mod git {
    fn entry(key: &str) -> Option<String> {
        let config = git2::Config::open_default().ok()?;
        let entry = config.get_entry(key).ok()?;
        if entry.has_value() {
            Some(entry.value()?.to_owned())
        } else {
            None
        }
    }

    /// Fetches the git username.
    pub(crate) fn username() -> Option<String> {
        entry("user.name")
    }

    /// Fetches the git email.
    pub(crate) fn email() -> Option<String> {
        entry("user.email")
    }

    pub(crate) fn pull(repo: &git2::Repository) -> Result<(), git2::Error> {
        let mut remote = repo.find_remote("origin")?;
        let mut fetch_options = git2::FetchOptions::new();
        remote.fetch(&["main"], Some(&mut fetch_options), None)?;
        let fetch_head = repo.refname_to_id("FETCH_HEAD")?;
        let mut reference = repo.find_reference("refs/heads/main")?;
        reference.set_target(fetch_head, "Updating to latest commit")?;
        let mut checkout_builder = git2::build::CheckoutBuilder::new();
        repo.checkout_head(Some(&mut checkout_builder.force()))?;
        Ok(())
    }

    /// Checkout to tag `tag_name`.
    pub(crate) fn checkout_tag(repo: git2::Repository, tag_name: &str) -> Result<(), git2::Error> {
        let reference = repo.find_reference(&format!("refs/tags/{}", tag_name))?;
        let tag_object = reference.peel(git2::ObjectType::Commit)?;
        repo.checkout_tree(&tag_object, None)?;
        repo.set_head_detached(tag_object.id())?;
        Ok(())
    }
}

pub(crate) mod fs {
    use std::{
        fmt, fs,
        io::{self, Read, Seek, SeekFrom},
        path::{Path, PathBuf},
    };

    use regex::Regex;

    pub(crate) fn copy_included_dir(
        source: &include_dir::Dir,
        destination: &Path,
    ) -> io::Result<()> {
        for entry in source.entries() {
            if let Some(file) = entry.as_file() {
                let dest = destination.join(file.path());
                if !dest.is_file() {
                    // NOTE: We cannot make use of fs::copy as the path may not make much sense
                    //       at runtime. The file contents however are embedded at compile time
                    //       and thus accessable during runtime.
                    fs::write(dest, file.contents())?;
                }
            }
            if let Some(dir) = entry.as_dir() {
                let dest = destination.join(dir.path());
                if !dest.is_dir() {
                    fs::create_dir(dest)?;
                }
                copy_included_dir(dir, destination)?;
            }
        }
        Ok(())
    }

    #[derive(Debug)]
    pub(crate) enum CopyTemplateError {
        IO(io::Error),
        Tera(tera::Error),
    }

    impl From<io::Error> for CopyTemplateError {
        fn from(value: io::Error) -> Self {
            Self::IO(value)
        }
    }

    impl From<tera::Error> for CopyTemplateError {
        fn from(value: tera::Error) -> Self {
            Self::Tera(value)
        }
    }

    impl fmt::Display for CopyTemplateError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::IO(x) => x.to_string(),
                    Self::Tera(x) => x.to_string(),
                }
            )
        }
    }

    impl std::error::Error for CopyTemplateError {}

    /// Copies a directory from `source` to `destination` by applying template
    /// rendering.
    pub(crate) fn copy_template_dir_with_rendering(
        source: &Path,
        destination: &Path,
        destination_root: &Path,
        context: &tera::Context,
        exclude_paths: &Vec<Regex>,
    ) -> Result<(), CopyTemplateError> {
        'top_level_dir: for entry in source.read_dir()? {
            let entry = entry?;

            let abs_path = entry.path();
            let path = abs_path.strip_prefix(source).unwrap();
            let output_path = destination.join(path);
            let mut path_relative_to_root = output_path
                .strip_prefix(destination_root)
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .into_owned();
            if abs_path.is_dir() {
                path_relative_to_root.push('/');
            }

            log::trace!("destination = {:?}", destination);
            log::trace!("destination_root = {:?}", destination_root);
            log::trace!("abs_path = {:?}", abs_path);
            log::trace!("path = {:?}", path);
            log::trace!("output_path = {:?}", output_path);
            log::trace!("path_relative_to_root = {:?}", path_relative_to_root);

            for ignore_pattern in exclude_paths {
                if ignore_pattern.is_match(&path_relative_to_root) {
                    log::trace!(
                        "Ignoring {} because it matches {}",
                        path_relative_to_root,
                        ignore_pattern
                    );
                    continue 'top_level_dir;
                }
            }

            if abs_path.is_dir() {
                fs::create_dir(&output_path)?;
                copy_template_dir_with_rendering(
                    &abs_path,
                    &output_path,
                    destination_root,
                    context,
                    exclude_paths,
                )?;
            } else if abs_path.is_file() {
                if abs_path
                    .file_name()
                    .is_some_and(|name| name == "template_metadata.ron")
                {
                    continue;
                }
                let output = fs::File::create(&output_path)?;

                if uses_tera(&abs_path)? {
                    let mut tera = tera::Tera::default();
                    tera.add_template_file(&abs_path, Some("t"))?;
                    tera.render_to("t", context, output)?;
                } else {
                    fs::copy(abs_path, output_path)?;
                }
            }
        }
        Ok(())
    }

    /// Returns `true` when the file contents start with the template indicator.
    fn uses_tera(path: &Path) -> io::Result<bool> {
        const TEMPLATE_INDICATOR: &str = "{# using tera #}";
        let mut file = fs::File::open(path)?;
        file.seek(SeekFrom::Start(0))?;
        let mut indicator: Vec<u8> = Vec::new();
        file.take(const { TEMPLATE_INDICATOR.len() as u64 })
            .read_to_end(&mut indicator)?;
        Ok(String::from_utf8_lossy(&indicator) == TEMPLATE_INDICATOR)
    }
}
