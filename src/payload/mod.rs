pub trait Payload {
    fn build(&mut self, packet: &mut [u8]) -> crate::Result<()>;
}

pub mod arp;
pub mod syn;

mod constant;
pub use constant::*;
