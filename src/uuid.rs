//! Manage and store UUIDs.
//!
//! To edit UUIDs, the `uuid` subcommand should be used instead of manually editing the file.
//!
//! # Examples
//!
//! Below is an example of how a UUID file could look like.
//!
//! ```toml
//! [BP]
//! header = "5fb4f7e8-4542-4dad-8e8b-ea43c3521c41"
//! module = "500c2b6a-6c8b-4423-9236-65e48ff76ab0"
//! dependencies = ["eb7d9bca-ad5d-4160-a86d-65ce8122daa4", "d64870ce-ebd6-453c-8082-0273fdb9a912"]
//!
//! [RP]
//!
//! [SP]
//!
//! [WT]
//! ```

use crate::Pack;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Data {
    pub header: Option<libuuid::Uuid>,
    pub module: Option<libuuid::Uuid>,
    pub deps: Vec<libuuid::Uuid>,
}

impl Data {
    /// Generates new randomly generated UUIDs for the header and module section.
    ///
    /// Using [`Data::default`] is preferred for generating a new project.
    pub fn generate() -> Self {
        Self {
            header: Some(libuuid::Uuid::new_v4()),
            module: Some(libuuid::Uuid::new_v4()),
            deps: Vec::new(),
        }
    }

    /// Updates the header UUID with `uuid` or generates a new one if `uuid` is [`None`].
    ///
    /// `header` is ensured to be [`Some`] after this operation.
    pub fn update_header(&mut self, uuid: Option<libuuid::Uuid>) -> &mut Self {
        self.header = Some(match uuid {
            Some(x) => x,
            None => libuuid::Uuid::new_v4(),
        });
        self
    }

    /// Updates the module UUID with `uuid` or generates a new one if `uuid` is [`None`].
    ///
    /// `header` is ensured to be [`Some`] after this operation.
    pub fn update_module(&mut self, uuid: Option<libuuid::Uuid>) -> &mut Self {
        self.module = Some(match uuid {
            Some(x) => x,
            None => libuuid::Uuid::new_v4(),
        });
        self
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Uuids {
    pub bp: Data,
    pub rp: Data,
    pub sp: Data,
    pub wt: Data,
}

impl Into<Table> for Uuids {
    fn into(self) -> Table {
        let mut table = Table::new();
        table.add_row(row![b => "", "Header", "Module", "Dependencies"]);
        for (name, pack) in [
            ("BP", self.bp),
            ("RP", self.rp),
            ("SP", self.sp),
            ("WT", self.wt),
        ] {
            table.add_row(row![
                name,
                pack.header.map(|u| u.to_string()).unwrap_or_default(),
                pack.module.map(|u| u.to_string()).unwrap_or_default(),
                pack.deps
                    .iter()
                    .map(|u| u.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ]);
        }
        table
    }
}

impl Uuids {
    /// Generates new UUIDs by using [`Data::generate`] for each pack.
    pub fn generate() -> Self {
        Self {
            bp: Data::generate(),
            rp: Data::generate(),
            sp: Data::generate(),
            wt: Data::generate(),
        }
    }

    pub fn of(&self, pack: &Pack) -> Data {
        match pack {
            Pack::Behavior => self.bp.clone(),
            Pack::Resource => self.rp.clone(),
            Pack::Skin => self.sp.clone(),
            Pack::WorldTemplate => self.wt.clone(),
        }
    }

    /// Loads UUIDs from TOML string.
    pub fn from_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
}

impl fmt::Display for Uuids {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", toml::to_string(self).unwrap())
    }
}
