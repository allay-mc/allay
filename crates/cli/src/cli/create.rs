use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
    process::{self, ExitCode},
    str::FromStr,
};

use allay::{localization, project::health::generate_project_id};
use clap::{ArgMatches, Command};
use console::style;
#[cfg(feature = "git")]
use inquire::list_option::ListOption;
use inquire::validator;
use slugify::slugify;

use super::ext::*;
#[cfg(feature = "git")]
use crate::resources::{self, MinecraftVersion};
use crate::{templates, utils};

pub(crate) fn cmd() -> Command {
    Command::new("create")
        .about("Create a new Allay project interactively")
        .arg_init_opts()
}

pub(crate) fn run(matches: &ArgMatches) -> process::ExitCode {
    // TODO: use raw_prompt for selection prompts to get the index directly
    // TODO: style messages in prompt if possible

    let manual_target_dir: Option<&PathBuf> = matches.get_one("init-target");
    #[cfg(feature = "git")]
    let init_with_git = matches.get_flag("init-git");

    let project_name = match inquire::Text::new("Project name")
        .with_placeholder("My Furniture")
        .with_validator(validator::ValueRequiredValidator::default())
        .prompt()
    {
        Ok(value) => value,
        Err(error) => {
            log::error!("{}", error);
            return ExitCode::FAILURE;
        }
    };

    let automatic_target_dir = PathBuf::from(slugify(&project_name, "", "_", None));
    let target_dir = manual_target_dir.unwrap_or(&automatic_target_dir);

    if target_dir
        .read_dir()
        .is_ok_and(|mut read_dir| read_dir.next().is_some())
    {
        log::error!("A directory named {} exists and is not empty; use `--directory` to specify the target directory manually", target_dir.display());
        return ExitCode::FAILURE;
    } else if !target_dir.exists() {
        log::trace!("Directory does not exist");
        if let Err(error) = fs::create_dir(target_dir) {
            log::error!("{}", error);
            return ExitCode::FAILURE;
        }
    } else {
        log::trace!("Directory exists and is empty");
    }

    let project_desc = match inquire::Text::new("Project description").prompt() {
        Ok(value) => value,
        Err(error) => {
            log::error!("{}", error);
            return ExitCode::FAILURE;
        }
    };

    let semver_validator = |value: &str| match semver::Version::from_str(value) {
        Ok(_) => Ok(inquire::validator::Validation::Valid),
        Err(error) => Ok(inquire::validator::Validation::Invalid(error.into())),
    };
    let project_version = match inquire::Text::new("Initial version of project")
        .with_default("0.1.0")
        .with_validator(semver_validator)
        .prompt()
    {
        Ok(value) => value,
        Err(error) => {
            log::error!("{}", error);
            return ExitCode::FAILURE;
        }
    };

    #[cfg(feature = "git")]
    let default_author_name: Option<String> = utils::git::username();
    #[cfg(not(feature = "git"))]
    let default_author_name: Option<String> = None;
    let project_author_name = {
        let mut prompt = inquire::Text::new("Project author")
            .with_placeholder("John Doe")
            .with_help_message("More authors can be added later on")
            .with_validator(validator::ValueRequiredValidator::default());
        if let Some(author) = &default_author_name {
            prompt = prompt.with_default(author);
        }
        match prompt.prompt() {
            Ok(value) => value,
            Err(error) => {
                log::error!("{}", error);
                return ExitCode::FAILURE;
            }
        }
    };

    #[cfg(feature = "git")]
    let default_author_email: Option<String> = utils::git::email();
    #[cfg(not(feature = "git"))]
    let default_author_email: Option<String> = None;
    let project_author_email = {
        let mut prompt =
            inquire::Text::new("Project author (email)").with_placeholder("john.doe@example.org");

        if let Some(email) = &default_author_email {
            prompt = prompt.with_default(email);
        }
        match prompt.prompt() {
            Ok(value) => value,
            Err(error) => {
                log::error!("{}", error);
                return ExitCode::FAILURE;
            }
        }
    };

    let mut languages = localization::Language::vanilla().to_vec();
    languages.sort_by_key(|language| language.as_str());
    let starting_cursor = &languages
        .binary_search_by_key(
            &localization::Language::NorthAmericaEnglish.as_str(),
            |language| language.as_str(),
        )
        .unwrap();
    let languages_pretty: Vec<String> = languages
        .iter()
        .map(|language| format!("{} {}", language.flag(), language.as_str()))
        .collect();
    let primary_language_pretty =
        match inquire::Select::new("Primary language", languages_pretty.clone())
            .with_starting_cursor(*starting_cursor)
            .prompt()
        {
            Ok(value) => value,
            Err(error) => {
                log::error!("{}", error);
                return ExitCode::FAILURE;
            }
        };
    let primary_language = languages[languages_pretty
        .iter()
        .position(|x| *x == primary_language_pretty)
        .unwrap()];

    let mut templates = templates::fetch_templates();
    templates.sort_by_key(|template| template.to_string());
    let template: Option<templates::Template> = if !templates.is_empty() {
        Some(
            match inquire::Select::new("Project template", templates).prompt() {
                Ok(value) => value,
                Err(error) => {
                    log::error!("{}", error);
                    return ExitCode::FAILURE;
                }
            },
        )
    } else {
        None
    };

    #[cfg(feature = "git")]
    // TODO: factor out the default somewhere more centralized and perhaps then
    //       embed with include_str!() macro
    let versions = resources::minecraft_versions().unwrap_or(vec![MinecraftVersion {
        version: "1.21.40".to_string(),
        date: "22-10-2024".to_string(),
        is_latest: false,
    }]);
    #[cfg(feature = "git")]
    let min_engine_version_index = match inquire::Select::new(
        "Minimum engine version",
        versions
            .iter()
            .map(|ver| {
                format!(
                    "{} {}{}",
                    ver.version,
                    ver.date,
                    if ver.is_latest { " (latest)" } else { "" }
                )
            })
            .collect(),
    )
    .raw_prompt()
    {
        Ok(ListOption { index, value: _ }) => index,
        Err(error) => {
            log::error!("{}", error);
            return ExitCode::FAILURE;
        }
    };
    #[cfg(feature = "git")]
    let min_engine_version: &String = &versions[min_engine_version_index].version;

    let mut licenses = License::ALL.to_vec();
    licenses.sort_by_key(|license| license.as_str());
    let default_license = licenses
        .binary_search_by_key(&License::Mit.as_str(), |template| template.as_str())
        .unwrap();
    let license: License = match inquire::Select::new("Project license", licenses)
        .with_starting_cursor(default_license)
        .prompt()
    {
        Ok(value) => value,
        Err(error) => {
            log::error!("{}", error);
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = fs::create_dir(allay::paths::project::internal(target_dir)) {
        log::error!("{}", error);
    }
    if let Err(error) = fs::create_dir(allay::paths::project::logs(target_dir)) {
        log::error!("{}", error);
    }
    if let Err(error) = fs::create_dir(allay::paths::project::build(target_dir)) {
        log::error!("{}", error);
    }
    if let Err(error) = fs::create_dir(allay::paths::project::db(target_dir)) {
        log::error!("{}", error);
    }
    if let Err(error) = fs::write(
        allay::paths::project::project_id(target_dir),
        generate_project_id().as_bytes(),
    ) {
        log::error!("{}", error);
    }

    if let Some(template) = template {
        // TODO: project_min_engine_version may be something like 1.21.80.3 so either accept it or
        //       trim it
        let mut context = tera::Context::new();
        context.insert("allay_version", &allay::VERSION);
        context.insert("project_name", &project_name);
        context.insert("project_description", &project_desc);
        context.insert("project_version", &project_version);
        context.insert("project_author_name", &project_author_name);
        context.insert("project_author_email", &project_author_email);
        context.insert("project_primary_language", &primary_language.id());
        #[cfg(feature = "git")]
        context.insert("project_min_engine_version", &min_engine_version);
        context.insert("project_license", &license.spdx());
        generate_scaffolding(&template, target_dir, &context);
    }

    #[cfg(feature = "git")]
    if init_with_git && git2::Repository::open(target_dir).is_err() {
        match git2::Repository::init(target_dir) {
            Ok(_repo) => {
                log::info!("Initialized git repository");
            }
            Err(error) => {
                log::error!("Failed to initialize git repository: {}", error);
            }
        }
    }

    println!("To get started run the following commands:");
    let console_prompt = style("$").green();
    let target_is_cwd = match (env::current_dir(), fs::canonicalize(target_dir)) {
        (Ok(current_dir), Ok(target_dir)) => current_dir == target_dir,
        _ => false,
    };
    if !target_is_cwd {
        println!(
            "{} cd {}",
            console_prompt,
            if cfg!(unix) {
                shell_escape::unix::escape(target_dir.to_string_lossy())
            } else if cfg!(windows) {
                shell_escape::windows::escape(target_dir.to_string_lossy())
            } else {
                unreachable!("expected either a windows or unix system")
            }
        );
    }
    println!("{} allay build", console_prompt);

    ExitCode::SUCCESS
}

fn generate_scaffolding(
    template: &templates::Template,
    destination: &Path,
    context: &tera::Context,
) {
    let root = &template.path;
    if let Err(error) = utils::fs::copy_template_dir_with_rendering(
        root,
        destination,
        destination,
        context,
        &template.metadata.exclude,
    ) {
        log::error!("Error while copying template directory: {}", error);
    }
}

#[derive(Clone, Copy)]
enum License {
    AgplV3,
    GplV3,
    LgplV3,
    Mozilla2,
    Apache2,
    Mit,
    Boost,
    Unlicense,
    Isc,
    Bsd2Clause,
    Bsd3Clause,
    Cddl,
    Eclipse2,
    Proprietary,
}

impl License {
    pub(crate) const ALL: [Self; 14] = [
        Self::AgplV3,
        Self::GplV3,
        Self::LgplV3,
        Self::Mozilla2,
        Self::Apache2,
        Self::Mit,
        Self::Boost,
        Self::Unlicense,
        Self::Isc,
        Self::Bsd2Clause,
        Self::Bsd3Clause,
        Self::Cddl,
        Self::Eclipse2,
        Self::Proprietary,
    ];

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::AgplV3 => "GNU AGPLv3",
            Self::GplV3 => "GNU GPLv3",
            Self::LgplV3 => "GNU LGPLv3",
            Self::Mozilla2 => "Mozilla Public License 2.0",
            Self::Apache2 => "Apache License 2.0",
            Self::Mit => "MIT",
            Self::Boost => "Boost Software License 1.0",
            Self::Unlicense => "The Unlicense",
            Self::Isc => "ISC License",
            Self::Bsd2Clause => "2-Clause BSD License",
            Self::Bsd3Clause => "3-Clause BSD License",
            Self::Cddl => "Common Development and Distribution License 1.0",
            Self::Eclipse2 => "Eclipse Public License 2",
            Self::Proprietary => "Proprietary",
        }
    }

    pub(crate) fn spdx(&self) -> &'static str {
        match self {
            Self::AgplV3 => "AGPL-3.0-only",
            Self::GplV3 => "GPL-3.0-only",
            Self::LgplV3 => "LGPL-3.0-only",
            Self::Mozilla2 => "MPL-2.0",
            Self::Apache2 => "Apache-2.0",
            Self::Mit => "MIT",
            Self::Boost => "BSL-1.0",
            Self::Unlicense => "Unlicense",
            Self::Isc => "ISC",
            Self::Bsd2Clause => "BSD-2-Clause",
            Self::Bsd3Clause => "BSD-3-Clause",
            Self::Cddl => "CDDL-1.0",
            Self::Eclipse2 => "EPL-2.0",
            Self::Proprietary => "Proprietary",
        }
    }
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
