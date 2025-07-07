// TODO: add github workflows for building to templates

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::{Files, SimpleFile},
    term::{self, termcolor::StandardStream},
};
use regex::Regex;
use std::{fmt, fs, path::PathBuf, str::FromStr};

pub const MAX_SUPPORTED_FORMAT_VERSION: semver::Version = semver::Version::new(1, 0, 0);

// Metadata of a template.
//
// Note that a metadat file may contain keys thet are not present in this struct. This allows
// forwards compatible in case new keys will be added in the future.
#[derive(Clone, serde::Deserialize)]
pub(crate) struct TemplateMetadata {
    /// The format version used by `template_metadata.ron`.
    ///
    /// This field is used for backwards compability.
    pub(crate) format_version: String,

    /// The name of the template.
    pub(crate) name: String,

    /// The version of the template.
    pub(crate) version: String,

    /// A namespace that can be used for organizing templates.
    ///
    /// This could be your company name for example.
    pub(crate) namespace: String,

    /// A one-sentence description of the template.
    pub(crate) description: String,

    /// A set of authors who contributed to the template.
    pub(crate) authors: Vec<String>,

    /// Regular Expression patterns of files/directories to not copy when creating new project.
    ///
    /// The file `template_metadata.ron` file is never copied so it is not necessary to include it
    /// in this field.
    #[serde(default)]
    #[serde(with = "serde_regex")]
    pub(crate) exclude: Vec<Regex>,
}

impl fmt::Display for TemplateMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} {} - {} by {}",
            self.namespace,
            self.name,
            self.version,
            self.description,
            self.authors.join(", ")
        )
    }
}

#[derive(Clone)]
pub(crate) struct Template {
    /// Metadata specified in `template_metadata.ron`.
    pub(crate) metadata: TemplateMetadata,

    /// The path to the template.
    pub(crate) path: PathBuf,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.metadata)
    }
}

/// Fetches all templates from `allay/templates/` in the configuration directory.
pub(crate) fn fetch_templates() -> Vec<Template> {
    let mut templates = Vec::new();
    let templates_dir = allay::paths::global::templates();
    if !templates_dir.is_dir() {
        log::error!("Missing template directory");
        return templates;
    }
    match templates_dir.read_dir() {
        Ok(entries) => {
            for entry in entries {
                let template_dir = match entry {
                    Ok(entry) => entry.path(),
                    Err(error) => {
                        log::error!("{}", error);
                        continue;
                    }
                };
                if template_dir.is_dir() {
                    let template_config_file = template_dir.join("template_metadata.ron");
                    if template_config_file.is_file() {
                        match fs::read_to_string(&template_config_file) {
                            Ok(content) => {
                                let metadata: TemplateMetadata = match ron::from_str(&content) {
                                    Ok(metadata) => metadata,
                                    Err(error) => {
                                        let simple_file =
                                            SimpleFile::new("template_metadata.ron", &content);
                                        let diagnostic = Diagnostic::error()
                                            .with_message(
                                                "Error while parsing template_metadata.ron",
                                            )
                                            .with_code(error.code.to_string())
                                            .with_labels(vec![Label::primary(
                                                (),
                                                simple_file
                                                    .line_range((), error.position.line - 1)
                                                    .unwrap(),
                                            )]);

                                        let writer = StandardStream::stderr(
                                            codespan_reporting::term::termcolor::ColorChoice::Auto,
                                        );
                                        let config = codespan_reporting::term::Config::default();

                                        if let Err(error) = term::emit(
                                            &mut writer.lock(),
                                            &config,
                                            &simple_file,
                                            &diagnostic,
                                        ) {
                                            log::error!("Error while trying to display error of template_metadata file: {}", error);
                                        };
                                        continue;
                                    }
                                };

                                let format_version =
                                    match semver::Version::from_str(&metadata.format_version) {
                                        Ok(ver) => ver,
                                        Err(error) => {
                                            log::error!(
                                                "Error while trying to parse format version: {}",
                                                error
                                            );
                                            continue;
                                        }
                                    };
                                if format_version.major > MAX_SUPPORTED_FORMAT_VERSION.major {
                                    log::warn!("The format of the template {} is not compatible: {} >> {}; try to update Allay or downgrade the template", metadata, format_version, MAX_SUPPORTED_FORMAT_VERSION);
                                    continue;
                                }

                                for tmpl in &templates {
                                    if tmpl.metadata.name == metadata.name
                                        && tmpl.metadata.namespace == metadata.namespace
                                    {
                                        log::error!("Both templates {} and {} share the same namespace and name", template_dir.display(), tmpl.path.display());
                                        continue;
                                    }
                                }
                                let template = Template {
                                    metadata,
                                    path: template_dir,
                                };

                                templates.push(template);
                            }
                            Err(error) => {
                                log::error!(
                                    "Failed to read template metadata at {}: {}",
                                    template_config_file.display(),
                                    error
                                );
                            }
                        }
                    }
                }
            }
        }
        Err(error) => {
            log::error!("{}", error);
        }
    };
    templates
}
