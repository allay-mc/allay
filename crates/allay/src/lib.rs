#![warn(missing_docs)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::expect_used)]

#![doc(html_favicon_url = "https://raw.githubusercontent.com/allay-mc/assets/main/logo-1080x.png")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/allay-mc/assets/main/logo-1080x.png")]
#![doc = include_str!("../../../README.md")]

pub mod plugin;
pub mod config;
pub mod localization;
pub mod lock;
pub mod manifest;
pub mod pack;
pub mod paths;
pub mod project;

pub use pack::Pack;
pub use project::{Project, BuildContext};
pub use config::Config;

/// The full version of Allay.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

