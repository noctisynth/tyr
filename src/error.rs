use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("OsError: {0}")]
    OsError(#[from] std::io::Error),
    #[error("Interface not found")]
    InterfaceNotFound,
    #[error("Buffer too small")]
    InsufficientBuffer,
    #[error("Invalid interface: {0}")]
    InvalidInterface(String),
}
