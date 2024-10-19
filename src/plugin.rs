use crate::config;
use std::ffi::OsStr;
use std::process::{Command, Output};

pub trait Plugin {
    /// The optional name of the plugin.
    fn name(&self) -> Option<String>;

    fn run<I, K, V>(&self, env_vars: I) -> Result<(String, Output), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>;

    fn panic(&self) -> bool;
}

pub struct ExecutablePlugin {
    /// Name of plugin.
    pub name: Option<String>,

    /// Path to executable.
    pub program: String,

    /// Arguments passed to executable.
    pub args: Vec<String>,

    pub panic: bool,
}

impl From<&config::Plugin> for ExecutablePlugin {
    fn from(value: &config::Plugin) -> Self {
        ExecutablePlugin {
            name: value.name.clone(),
            program: value.with.clone().unwrap_or(value.run.clone()),
            args: {
                let mut args: Vec<String> = Vec::new();
                if value.with.is_some() {
                    args.push(value.run.clone());
                };
                match value.args.clone().unwrap_or_default() {
                    config::PluginArgs::Options(options) => {
                        args.push(
                            serde_json::to_string(&options)
                                .expect("failed to transform TOML to JSON"),
                        );
                    }
                    config::PluginArgs::Args(a) => args.extend(a.to_vec()),
                };
                args
            },
            panic: value.panic,
        }
    }
}

impl Plugin for ExecutablePlugin {
    fn name(&self) -> Option<String> {
        self.name.clone()
    }

    #[must_use]
    fn run<I, K, V>(&self, env_vars: I) -> Result<(String, Output), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let mut cmd = Command::new(&self.program);
        let cmd = cmd.args(&self.args).envs(env_vars);
        let output = cmd.output()?;
        let name = self.name().unwrap_or("<unnamed>".to_string());
        Ok((name, output))
    }

    fn panic(&self) -> bool {
        self.panic
    }
}
