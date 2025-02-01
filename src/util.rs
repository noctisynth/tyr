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

/// Retrieves the network interface with the default gateway.
///
/// # Returns
///
/// * `Option<datalink::NetworkInterface>` - An option containing the network interface with the default gateway if found, or None if not found.
pub fn get_default_interface() -> Option<datalink::NetworkInterface> {
    datalink::interfaces().into_iter().find(|interface| {
        interface
            .ips
            .iter()
            .any(|ip| ip.is_ipv4() && ip.ip().is_loopback())
    })
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

/// Calculates the number of threads to use based on the rated power.
///
/// The rated power is a value from 1 to 4, which mean use a fraction
/// of the available threads.
///
/// # Arguments
///
/// * `rated_power` - The rated power value.
///
/// # Returns
///
/// The number of threads to use.
///
/// # Errors
///
/// Returns an error if the system fails to determine the available parallelism.
///
/// # Panics
///
/// Panics if the rated power is not in the range 1 to 4.
///
/// # Warning
///
/// 0 is also a legal power rating parameter. Using the zero rated power will
/// put the unit into a full load condition, which will definitely affect the
/// normal operation of the unit and may even cause unknown consequences.
///
/// **USE AT YOUR OWN RISK!**
pub fn get_num_threads(rated_power: u8) -> crate::Result<usize> {
    let available_parallelism = std::thread::available_parallelism()?.get();
    Ok(match rated_power {
        0 => available_parallelism,
        1 => available_parallelism / 10,
        2 => available_parallelism / 5,
        3 => available_parallelism / 2,
        4 => available_parallelism * 3 / 4,
        _ => panic!(
            "Invalid rated power {}, available ratings are 1 to 4",
            rated_power
        ),
    })
}
