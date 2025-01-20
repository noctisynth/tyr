use crate::Result;

const ETHERNET_HEADER_LEN: usize = 14;
const IPV4_HEADER_LEN: usize = 20;

pub trait Payload {
    fn build(&mut self, packet: &mut [u8]) -> Result<()>;
}

pub mod syn;
