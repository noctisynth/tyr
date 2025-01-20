use std::net::Ipv4Addr;

pub fn get_random_port() -> u16 {
    fastrand::u16(1024..65535)
}

pub fn get_random_ip() -> Ipv4Addr {
    Ipv4Addr::new(
        fastrand::u8(0..255),
        fastrand::u8(0..255),
        fastrand::u8(0..255),
        fastrand::u8(0..255),
    )
}
