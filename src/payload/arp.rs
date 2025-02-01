use std::net::Ipv4Addr;

use pnet::packet::{arp, ethernet};
use pnet::{datalink, util::MacAddr};

pub struct ArpPayload {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub interface: datalink::NetworkInterface,
}

impl super::Payload for ArpPayload {
    fn build(&mut self, packet: &mut [u8]) -> crate::Result<()> {
        {
            let mut ethernet_header =
                ethernet::MutableEthernetPacket::new(&mut packet[..super::PKT_ETH_SIZE])
                    .ok_or(crate::error::Error::InsufficientBuffer)?;
            ethernet_header.set_destination(MacAddr::broadcast());
            ethernet_header.set_source(self.interface.mac.ok_or(
                crate::error::Error::InvalidInterface(format!(
                    "Failed to get MAC address for interface {}",
                    self.interface.name,
                )),
            )?);
            ethernet_header.set_ethertype(ethernet::EtherTypes::Arp);
        }

        {
            let mut arp_header = arp::MutableArpPacket::new(&mut packet[super::PKT_ETH_SIZE..])
                .ok_or(crate::error::Error::InsufficientBuffer)?;

            arp_header.set_hardware_type(arp::ArpHardwareTypes::Ethernet);
            arp_header.set_protocol_type(ethernet::EtherTypes::Ipv4);
            arp_header.set_hw_addr_len(6);
            arp_header.set_proto_addr_len(4);
            arp_header.set_operation(arp::ArpOperations::Request);
            arp_header.set_sender_hw_addr(self.interface.mac.ok_or(
                crate::error::Error::InvalidInterface(format!(
                    "Failed to get MAC address for interface {}",
                    self.interface.name,
                )),
            )?);
            arp_header.set_sender_proto_addr(self.src_ip);
            arp_header.set_target_hw_addr(MacAddr::zero());
            arp_header.set_target_proto_addr(self.dst_ip);
        }

        Ok(())
    }
}
