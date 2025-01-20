use std::{net::Ipv4Addr, os::unix::process::ExitStatusExt};

use pnet::datalink;

use crate::error;

/// Generates a random port number within the range of 1024 to 65535.
///
/// # Returns
///
/// A `u16` representing a random port number.
pub fn get_random_port() -> u16 {
    fastrand::u16(1024..65535)
}

/// Generates a random IPv4 address.
///
/// # Returns
///
/// An `Ipv4Addr` representing a random IPv4 address.
pub fn get_random_ip() -> Ipv4Addr {
    Ipv4Addr::new(
        fastrand::u8(0..255),
        fastrand::u8(0..255),
        fastrand::u8(0..255),
        fastrand::u8(0..255),
    )
}

/// Retrieves a network interface by its name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the network interface.
///
/// # Returns
///
/// * `Option<datalink::NetworkInterface>` - An option containing the network interface if found, or None if not found.
pub fn get_interface(name: &str) -> Option<datalink::NetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .find(|interface| interface.name == name)
}

/// Retrieves all network interfaces available on the system.
///
/// # Returns
///
/// * `Vec<datalink::NetworkInterface>` - A vector containing all network interfaces.
pub fn get_interfaces() -> Vec<datalink::NetworkInterface> {
    datalink::interfaces()
}

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
pub fn rerun_if_not_root() -> crate::Result<()> {
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
