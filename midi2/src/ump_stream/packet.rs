use crate::{error, ump_stream};

#[derive(Eq, PartialEq, Clone, midi2_proc::Debug)]
pub struct Packet(pub(crate) [u32; 4]);

impl crate::traits::BufferAccess<[u32; 4]> for Packet {
    fn buffer_access(&self) -> &[u32; 4] {
        &self.0
    }
    fn buffer_access_mut(&mut self) -> &mut [u32; 4]
    where
        [u32; 4]: crate::buffer::BufferMut,
    {
        &mut self.0
    }
}

impl<'a> core::convert::TryFrom<&'a [u32]> for Packet {
    type Error = error::InvalidData;
    fn try_from(data: &'a [u32]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT,
            ));
        }

        use crate::detail::BitOps;
        if u8::from(data[0].nibble(0)) != ump_stream::UMP_MESSAGE_TYPE {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE,
            ));
        }

        Ok(Packet({
            let mut buffer = [0x0; 4];
            let sz = 4.min(data.len());
            buffer[..sz].copy_from_slice(&data[..sz]);
            buffer
        }))
    }
}

impl core::ops::Deref for Packet {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Format {
    Complete,
    Start,
    Continue,
    End,
}

impl Packet {
    pub fn format(&self) -> Format {
        format_from_data(&self.0[..])
    }
}

fn format_from_data(data: &[u32]) -> Format {
    use crate::detail::BitOps;
    use Format::*;
    match u8::from(data[0].crumb(2)) {
        0x0 => Complete,
        0x1 => Start,
        0x2 => Continue,
        0x3 => End,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn construction() {
        assert!(Packet::try_from(&[0xF000_0000][..]).is_ok())
    }

    #[test]
    fn construction_long_slice() {
        assert!(Packet::try_from(&[0xF000_0000, 0x0, 0x0, 0x0][..]).is_ok())
    }

    #[test]
    fn construction_very_long_slice() {
        assert!(Packet::try_from(&[0xF000_0000, 0x0, 0x0, 0x0, 0x0][..]).is_ok())
    }

    #[test]
    fn construction_short_slice() {
        assert_eq!(
            Packet::try_from(&[][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT
            )),
        )
    }

    #[test]
    fn construction_incorrect_ump_message_type() {
        assert_eq!(
            Packet::try_from(&[0x0000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            )),
        )
    }

    #[test]
    fn complete_format() {
        assert_eq!(
            Packet::try_from(&[0xF000_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .format(),
            Format::Complete,
        )
    }

    #[test]
    fn start_format() {
        assert_eq!(
            Packet::try_from(&[0xF400_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .format(),
            Format::Start,
        )
    }

    #[test]
    fn continue_format() {
        assert_eq!(
            Packet::try_from(&[0xF800_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .format(),
            Format::Continue,
        )
    }

    #[test]
    fn end_format() {
        assert_eq!(
            Packet::try_from(&[0xFC00_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000][..])
                .unwrap()
                .format(),
            Format::End,
        )
    }
}
