#![doc(html_favicon_url = "https://raw.githubusercontent.com/allay-mc/assets/main/logo-1080x.png")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/allay-mc/assets/main/logo-1080x.png")]
#![doc = include_str!("../README.md")]

pub mod config;
pub mod diagnostic;
pub mod error;
pub mod filter;
pub mod health;
pub mod localization;
pub mod manifest;
pub mod pack;
pub mod paths;
pub mod plugin;
pub mod project;
pub mod scaffolding;
pub mod uuid;

pub use config::Config;
pub use error::Error;
pub use health::Health;
pub use manifest::Manifest;
pub use pack::Pack;
pub use project::Project;
