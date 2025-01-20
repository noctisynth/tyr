use pnet::datalink;

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
