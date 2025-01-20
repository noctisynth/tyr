use std::os::unix::process::ExitStatusExt;

pub mod error;
pub mod interface;
pub mod payload;
pub mod util;

/// A type alias for the `Result` type used throughout the library.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[inline(always)]
/// Checks if the current user is the root user.
///
/// Returns `true` if the current user is the root user, `false` otherwise.
fn is_root() -> bool {
    nix::unistd::Uid::current().is_root()
}

/// If the current user is not the root user, reruns the current program using sudo.
///
/// Returns `Ok(())` if the program was successfully rerun with sudo, or an `Err` if the rerun failed.
pub fn rerun_if_not_root() -> Result<()> {
    if !is_root() {
        let args: Vec<String> = std::env::args().collect();
        let program = &args[0];
        let rest_args = &args[1..];

        let status = std::process::Command::new("sudo")
            .arg(program)
            .args(rest_args)
            .status()?;

        if !status.success() {
            match status.signal() {
                Some(nix::libc::SIGINT) => {
                    std::process::exit(130);
                }
                Some(signal) => {
                    return Err(error::Error::OsError(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        format!("Failed to grant root privileges: signal {}", signal),
                    )));
                }
                _ => {
                    return Err(error::Error::OsError(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "Failed to grant root privileges",
                    )));
                }
            }
        }
    }
    Ok(())
}
