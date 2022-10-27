use crate::{
    error::Error,
    util::Truncate, 
    packet::{Packet, PacketMethods},
};
use super::super::helpers;

#[derive(
    Clone,
    Debug, 
    PartialEq,
)]
pub struct Message {
    group: ux::u4,
    time_code: ux::u7,
}

impl Message {
    pub const TYPE_CODE: ux::u4 = super::TYPE_CODE;
    pub const STATUS_CODE: u8 = 0xF1;
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(
            &p,
            Message::STATUS_CODE,
        )?;
        Ok(Message {
            group: helpers::group_from_packet(&p),
            time_code: p.octet(2).truncate(),
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        super::write_data_to_packet(
            &mut p, 
            m.group, 
            Message::STATUS_CODE, 
            Some(m.time_code),
            None
        );
        p
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
            Message::try_from(Packet::from_data(&[0x14F1_5300])),
            Ok(Message {
                group: ux::u4::new(0x4),
                time_code: ux::u7::new(0x53),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                time_code: ux::u7::new(0x2A),
            }),
            Packet::from_data(&[0x15F1_2A00]),
        );
    }
}
