//! Project skeleton for an Allay project.

/// The gitignore template.
pub const GITIGNORE: &[u8] = include_bytes!("gitignore");

/// The configuration file (allay.toml) template.
// TODO: use template engine to fill in info like git username in authors field
pub const CONFIG: &[u8] = include_bytes!("allay.toml");

/// The default pack icon.
pub const PACK_ICON: &[u8] = include_bytes!("pack_icon.png");
