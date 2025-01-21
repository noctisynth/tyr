pub trait Payload {
    fn build(&mut self, packet: &mut [u8]) -> crate::Result<()>;
}

pub mod syn;

mod constant;
pub use constant::*;
