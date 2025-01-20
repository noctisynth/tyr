use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    OsError(#[from] std::io::Error),
    InterfaceNotFound,
    PacketBufferTooSmall,
    InvalidInterface(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OsError(e) => write!(f, "OsError: {}", e),
            _ => write!(f, "{}", self),
        }
    }
}
