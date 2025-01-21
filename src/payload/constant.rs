use pnet::packet::icmpv6::ndp;
use pnet::packet::{arp, ethernet, ipv4, ipv6, tcp};

// Constants below are copied from https://github.com/libpnet/libpnet/blob/main/examples/ip_to_mac.rs
pub const PKT_ETH_SIZE: usize = ethernet::EthernetPacket::minimum_packet_size();
pub const PKT_ARP_SIZE: usize = arp::ArpPacket::minimum_packet_size();
pub const PKT_IPV4_SIZE: usize = ipv4::Ipv4Packet::minimum_packet_size();
pub const PKT_IPV6_SIZE: usize = ipv6::Ipv6Packet::minimum_packet_size();
pub const PKT_TCP_SIZE: usize = tcp::TcpPacket::minimum_packet_size();
pub const PKT_NDP_SOL_SIZE: usize = ndp::NeighborSolicitPacket::minimum_packet_size();
pub const PKT_NDP_ADV_SIZE: usize = ndp::NeighborAdvertPacket::minimum_packet_size();
pub const PKT_OPT_SIZE: usize = ndp::NdpOptionPacket::minimum_packet_size();
pub const PKT_MAC_SIZE: usize = 6;

pub const PKT_ARP_OFFSET: usize = PKT_ETH_SIZE;
pub const PKT_IPV6_OFFSET: usize = PKT_ETH_SIZE;
pub const PKT_NDP_OFFSET: usize = PKT_IPV6_OFFSET + PKT_IPV6_SIZE;
