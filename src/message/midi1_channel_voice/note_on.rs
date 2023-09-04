use crate::{
    message::{
        helpers as message_helpers,
        midi1_channel_voice::{helpers as midi1cv_helpers, TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE},
    },
    result::Result,
    util::debug,
    *,
};

const OP_CODE: u4 = u4::new(0b1001);

#[derive(Clone, PartialEq, Eq)]
pub struct NoteOnMessage<'a>(&'a [u32]);

debug::message_debug_impl!(NoteOnMessage);

impl<'a> NoteOnMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn note(&self) -> u7 {
        message_helpers::note_from_packet(self.0)
    }
    pub fn velocity(&self) -> u7 {
        midi1cv_helpers::note_velocity_from_packet(self.0)
    }
}

impl<'a> Message<'a> for NoteOnMessage<'a> {
    type Builder = NoteOnBuilder<'a>;
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI1_CHANNEL_VOICE_TYPE, OP_CODE)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> GroupedMessage<'a> for NoteOnMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct NoteOnBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> NoteOnBuilder<'a> {
    pub fn channel(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn note(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_note_to_packet(v, buffer);
        }
        self
    }
    pub fn velocity(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi1cv_helpers::write_note_velocity_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> Builder<'a> for NoteOnBuilder<'a> {
    type Message = NoteOnMessage<'a>;
    fn build(self) -> Result<NoteOnMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(NoteOnMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
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
}

impl<'a> GroupedBuilder<'a> for NoteOnBuilder<'a> {
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
            NoteOnMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xD))
                .channel(u4::new(0xE))
                .note(u7::new(0x75))
                .velocity(u7::new(0x3D))
                .build(),
            Ok(NoteOnMessage(&[0x2D9E_753D])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().group(),
            u4::new(0xD),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().note(),
            u7::new(0x75),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x2D9E_753D]).unwrap().velocity(),
            u7::new(0x3D),
        );
    }
}
