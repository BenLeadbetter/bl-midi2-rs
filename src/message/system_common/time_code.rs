use super::super::helpers;
use crate::{
    error::Error,
    message::Midi2Message,
    util::{builder, getter, BitOps, Truncate},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    group: ux::u4,
    time_code: ux::u7,
}

builder::builder!(group: ux::u4, time_code: ux::u7);

impl Message {
    const STATUS_CODE: u8 = 0xF1;
    getter::getter!(group, ux::u4);
    getter::getter!(time_code, ux::u7);
    builder::builder_method!();
}

impl Midi2Message for Message {
    fn validate_ump(bytes: &[u32]) -> Result<(), Error> {
        super::validate_packet(bytes, Message::STATUS_CODE)
    }
    fn from_ump(bytes: &[u32]) -> Self {
        Message {
            group: helpers::group_from_packet(bytes),
            time_code: bytes[0].octet(2).truncate(),
        }
    }
    fn to_ump<'a>(&self, bytes: &'a mut [u32]) -> &'a [u32] {
        super::write_data_to_packet(
            bytes,
            self.group,
            Message::STATUS_CODE,
            Some(self.time_code),
            None,
        );
        &bytes[..1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;

    message_traits_test!(Message);

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from_ump(&[0x14F1_5300]),
            Ok(Message {
                group: ux::u4::new(0x4),
                time_code: ux::u7::new(0x53),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Message {
                group: ux::u4::new(0x5),
                time_code: ux::u7::new(0x2A),
            }
            .to_ump(&mut [0x0]),
            &[0x15F1_2A00],
        );
    }
}
