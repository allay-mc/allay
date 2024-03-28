// TODO: impl errors

use std::fmt::Display;

pub const ERROR_PREFIX: char = 'E';
pub const WARNING_PREFIX: char = 'W';

pub trait Diagnostic {
    fn brief_description(&self) -> &str;
    fn extensive_description(&self) -> Option<&str>;
    fn code(self) -> u8;
    fn id(self) -> String;
    fn from_code(code: u8) -> Self;
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Warning {
    RedundantManifest = 1,
    RedundantPackIcon = 2,
    EmptyAddOn = 3,
}

impl Diagnostic for Warning {
    fn brief_description(&self) -> &str {
        match self {
            Self::RedundantManifest => {
                "Found `manifest.json` but ignoring it as `custom-manifest` is not set to `true`"
            }
            Self::RedundantPackIcon => {
                "Found `pack_icon.png` but ignoring it as `custom-pack-icon` is not set to `true`"
            }
            Self::EmptyAddOn => "Add-On contains no packs",
        }
    }

    fn extensive_description(&self) -> Option<&str> {
        match self {
            Self::RedundantManifest => Some(include_str!("redundant_manifest.md")),
            Self::RedundantPackIcon => Some(include_str!("redundant_pack_icon.md")),
            Self::EmptyAddOn => None,
        }
    }

    fn code(self) -> u8 {
        self as u8
    }

    fn id(self) -> String {
        format!("W{:03}", self.code())
    }

    fn from_code(code: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(code) }
    }
}

impl Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{id:03}] {brief_description}{extra}",
            id = self.id(),
            brief_description = self.brief_description(),
            extra = match self.extensive_description() {
                Some(_) => format!(
                    ". Use `allay explain W{}` to gather further information",
                    self.id()
                ),
                None => String::new(),
            }
        )
    }
}
