use crate::Pack;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot find project")]
    NotInAProject,

    #[error("Missing UUID for {0}")]
    MissingUuid(Pack),

    #[error("Invalid project setup")]
    InvalidProjectSetup,
}
