use crate::{util::BitOps, *};

pub mod no_op;
pub mod time_stamp;

pub use no_op::NoOpMessage;
pub use no_op::NoOpMessageBuilder;
pub use time_stamp::TimeStampMessage;
pub use time_stamp::TimeStampMessageBuilder;

pub fn validate_packet(p: &[u32], op_code: u4) -> Result<()> {
    if p.is_empty() {
        Err(Error::BufferOverflow)
    } else if p[0].nibble(0) != u4::new(0x0) || p[0].nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}
