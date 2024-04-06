use std::fmt::Display;

pub const ERROR_PREFIX: char = 'E';
pub const WARNING_PREFIX: char = 'W';

pub trait Diagnostic {
    fn brief_description(&self) -> &str;
    fn extensive_description(&self) -> Option<&str>;
    fn code(self) -> u8;
    fn id(self) -> String;
    fn from_code(code: u8) -> Option<Self>
    where
        Self: Sized;
    fn kind(self) -> Kind;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Error,
    Warning,
}

#[derive(Copy, Clone)]
pub enum Notification {
    RedundantManifest,
    RedundantPackIcon,
    EmptyAddOn,
    ComMojangNotFoundAndroid,
    ComMojangWindows,
}

impl Diagnostic for Notification {
    fn brief_description(&self) -> &str {
        match self {
            Self::RedundantManifest => {
                "Found `manifest.json` but ignoring it as `custom-manifest` is not set to `true`"
            }
            Self::RedundantPackIcon => {
                "Found `pack_icon.png` but ignoring it as `custom-pack-icon` is not set to `true`"
            }
            Self::EmptyAddOn => "Add-On contains no packs",
            Self::ComMojangNotFoundAndroid | Self::ComMojangWindows => {
                "The `com.mojang` folder cannot be found"
            }
        }
    }

    fn extensive_description(&self) -> Option<&str> {
        match self {
            Self::RedundantManifest => Some(include_str!("redundant_manifest.md")),
            Self::RedundantPackIcon => Some(include_str!("redundant_pack_icon.md")),
            Self::EmptyAddOn => None,
            Self::ComMojangNotFoundAndroid => Some(include_str!("com_mojang_not_found_android.md")),
            Self::ComMojangWindows => None,
        }
    }

    fn code(self) -> u8 {
        match self {
            Self::RedundantManifest => 1,
            Self::RedundantPackIcon => 2,
            Self::EmptyAddOn => 3,
            Self::ComMojangNotFoundAndroid => 4,
            Self::ComMojangWindows => 5,
        }
    }

    fn id(self) -> String {
        format!(
            "{}{:03}",
            match self.kind() {
                Kind::Error => ERROR_PREFIX,
                Kind::Warning => WARNING_PREFIX,
            },
            self.code()
        )
    }

    fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::RedundantManifest),
            2 => Some(Self::RedundantPackIcon),
            3 => Some(Self::EmptyAddOn),
            4 => Some(Self::ComMojangNotFoundAndroid),
            5 => Some(Self::ComMojangWindows),
            _ => None,
        }
    }

    fn kind(self) -> Kind {
        match self {
            Self::RedundantManifest => Kind::Warning,
            Self::RedundantPackIcon => Kind::Warning,
            Self::EmptyAddOn => Kind::Warning,
            Self::ComMojangNotFoundAndroid => Kind::Error,
            Self::ComMojangWindows => Kind::Error,
        }
    }
}

impl Display for Notification {
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
