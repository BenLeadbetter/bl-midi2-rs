use crate::{
    message::{
        helpers as message_helpers, midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE,
    },
    result::Result,
    util::debug,
    *,
};

const OP_CODE: u4 = u4::new(0b1101);

#[derive(Clone, PartialEq, Eq)]
pub struct ChannelPressureMessage<'a>(&'a [u32]);

debug::message_debug_impl!(ChannelPressureMessage);

impl<'a> ChannelPressureMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn pressure(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
}

impl<'a> Message<'a> for ChannelPressureMessage<'a> {
    type Builder = ChannelPressureBuilder<'a>;
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        Self(data)
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)
    }
    fn data(&self) -> &'a [u32] {
        self.0
    }
}

impl<'a> GroupedMessage<'a> for ChannelPressureMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct ChannelPressureBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> ChannelPressureBuilder<'a> {
    pub fn channel(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn pressure(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> Builder<'a> for ChannelPressureBuilder<'a> {
    type Message = ChannelPressureMessage<'a>;
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 1) {
            Ok(()) => {
                message_helpers::clear_buffer(&mut buffer[..1]);
                message_helpers::write_op_code_to_packet(OP_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI1_CHANNEL_VOICE_TYPE, buffer);
                Self(Ok(&mut buffer[..1]))
            }
            Err(e) => Self(Err(e)),
        }
    }
    fn build(self) -> Result<ChannelPressureMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(ChannelPressureMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<'a> GroupedBuilder<'a> for ChannelPressureBuilder<'a> {
    fn group(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_group_to_packet(v, buffer);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::random_buffer;

    #[test]
    fn builder() {
        assert_eq!(
            ChannelPressureMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xF))
                .channel(u4::new(0x6))
                .pressure(u7::new(0x09))
                .build(),
            Ok(ChannelPressureMessage(&[0x2FD6_0900])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900])
                .unwrap()
                .group(),
            u4::new(0xF),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900])
                .unwrap()
                .pressure(),
            u7::new(0x09),
        );
    }
}
