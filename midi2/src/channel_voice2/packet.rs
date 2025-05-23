use crate::{channel_voice2, error};

#[derive(Eq, PartialEq, Clone, midi2_proc::Debug)]
pub struct Packet(pub(crate) [u32; 2]);

impl crate::traits::BufferAccess<[u32; 2]> for Packet {
    fn buffer_access(&self) -> &[u32; 2] {
        &self.0
    }
    fn buffer_access_mut(&mut self) -> &mut [u32; 2]
    where
        [u32; 2]: crate::buffer::BufferMut,
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
        if u8::from(data[0].nibble(0)) != channel_voice2::UMP_MESSAGE_TYPE {
            return Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE,
            ));
        }

        Ok(Packet({
            let mut buffer = [0x0; 2];
            let sz = 2.min(data.len());
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

impl crate::Grouped<[u32; 2]> for Packet {
    fn group(&self) -> crate::ux::u4 {
        use crate::detail::BitOps;
        self.0[0].nibble(1)
    }
    fn set_group(&mut self, group: crate::ux::u4)
    where
        [u32; 2]: crate::buffer::BufferMut,
    {
        use crate::detail::BitOps;
        self.0[0].set_nibble(1, group);
    }
}

impl crate::Channeled<[u32; 2]> for Packet {
    fn channel(&self) -> crate::ux::u4 {
        use crate::detail::BitOps;
        self.0[0].nibble(3)
    }
    fn set_channel(&mut self, channel: crate::ux::u4)
    where
        [u32; 2]: crate::buffer::BufferMut,
    {
        use crate::detail::BitOps;
        self.0[0].set_nibble(3, channel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        assert!(Packet::try_from(&[0x4000_0000][..]).is_ok());
    }

    #[test]
    fn construction_slice_size2() {
        assert!(Packet::try_from(&[0x4000_0000, 0x0][..]).is_ok());
    }

    #[test]
    fn construction_slice_size4() {
        assert!(Packet::try_from(&[0x4000_0000, 0x0, 0x0, 0x0][..]).is_ok());
    }

    #[test]
    fn construction_incorrect_ump_message_type() {
        assert_eq!(
            Packet::try_from(&[0x0000_0000][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            )),
        );
    }

    #[test]
    fn construction_short_slice() {
        assert_eq!(
            Packet::try_from(&[][..]),
            Err(error::InvalidData(
                crate::detail::common_err_strings::ERR_SLICE_TOO_SHORT
            )),
        );
    }

    #[test]
    fn channel() {
        use crate::Channeled;
        assert_eq!(
            Packet::try_from(&[0x4008_0000][..]).unwrap().channel(),
            ux::u4::new(0x8)
        );
    }

    #[test]
    fn set_channel() {
        use crate::Channeled;
        let mut packet = Packet::try_from(&[0x4000_0000][..]).unwrap();
        packet.set_channel(ux::u4::new(0x8));
        assert_eq!(&*packet, &[0x4008_0000, 0x0][..]);
    }

    #[test]
    fn group() {
        use crate::Grouped;
        assert_eq!(
            Packet::try_from(&[0x4A00_0000][..]).unwrap().group(),
            ux::u4::new(0xA)
        );
    }

    #[test]
    fn set_group() {
        use crate::Grouped;
        let mut packet = Packet::try_from(&[0x4000_0000][..]).unwrap();
        packet.set_group(ux::u4::new(0xA));
        assert_eq!(&*packet, &[0x4A00_0000, 0x0][..]);
    }
}
