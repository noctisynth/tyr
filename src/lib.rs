pub mod error;
pub mod payload;
pub mod util;

/// A type alias for the `Result` type used throughout the library.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;
