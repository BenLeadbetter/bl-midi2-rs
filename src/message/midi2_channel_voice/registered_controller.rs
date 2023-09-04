use crate::{
    message::{
        helpers as message_helpers,
        midi2_channel_voice::{helpers as midi2cv_helpers, TYPE_CODE as MIDI2CV_TYPE_CODE},
    },
    result::Result,
    util::debug,
    *,
};

const TYPE_CODE: u4 = u4::new(0b0010);

#[derive(Clone, PartialEq, Eq)]
pub struct RegisteredControllerMessage<'a>(&'a [u32]);

debug::message_debug_impl!(RegisteredControllerMessage);

impl<'a> RegisteredControllerMessage<'a> {
    pub fn channel(&self) -> u4 {
        message_helpers::channel_from_packet(self.0)
    }
    pub fn bank(&self) -> u7 {
        midi2cv_helpers::controller_bank_from_packet(self.0)
    }
    pub fn index(&self) -> u7 {
        midi2cv_helpers::controller_index_from_packet(self.0)
    }
    pub fn controller_data(&self) -> u32 {
        midi2cv_helpers::controller_data_from_packet(self.0)
    }
}

impl<'a> Message<'a> for RegisteredControllerMessage<'a> {
    type Builder = RegisteredControllerBuilder<'a>;
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        message_helpers::validate_packet(buffer, MIDI2CV_TYPE_CODE, TYPE_CODE)?;
        message_helpers::validate_buffer_size(buffer, 2)?;
        Ok(())
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        Self(buffer)
    }
}

impl<'a> GroupedMessage<'a> for RegisteredControllerMessage<'a> {
    fn group(&self) -> u4 {
        message_helpers::group_from_packet(self.0)
    }
}

#[derive(PartialEq, Eq)]
pub struct RegisteredControllerBuilder<'a>(Result<&'a mut [u32]>);

impl<'a> RegisteredControllerBuilder<'a> {
    pub fn channel(mut self, v: u4) -> Self {
        if let Ok(buffer) = &mut self.0 {
            message_helpers::write_channel_to_packet(v, buffer);
        }
        self
    }
    pub fn bank(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_controller_bank_to_packet(v, buffer);
        }
        self
    }
    pub fn index(mut self, v: u7) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_controller_index_to_packet(v, buffer);
        }
        self
    }
    pub fn controller_data(mut self, v: u32) -> Self {
        if let Ok(buffer) = &mut self.0 {
            midi2cv_helpers::write_controller_data_to_packet(v, buffer);
        }
        self
    }
}

impl<'a> Builder<'a> for RegisteredControllerBuilder<'a> {
    type Message = RegisteredControllerMessage<'a>;
    fn build(self) -> Result<RegisteredControllerMessage<'a>> {
        match self.0 {
            Ok(buffer) => Ok(RegisteredControllerMessage(buffer)),
            Err(e) => Err(e.clone()),
        }
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        match message_helpers::validate_buffer_size(buffer, 2) {
            Ok(()) => {
                message_helpers::clear_buffer(&mut buffer[..2]);
                message_helpers::write_op_code_to_packet(TYPE_CODE, buffer);
                message_helpers::write_type_to_packet(MIDI2CV_TYPE_CODE, buffer);
                Self(Ok(&mut buffer[..2]))
            }
            Err(e) => Self(Err(e)),
        }
    }
}

impl<'a> GroupedBuilder<'a> for RegisteredControllerBuilder<'a> {
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
            RegisteredControllerMessage::builder(&mut random_buffer::<2>())
                .group(u4::new(0xA))
                .channel(u4::new(0xB))
                .bank(u7::new(0x7D))
                .index(u7::new(0x64))
                .controller_data(0x46845E00)
                .build(),
            Ok(RegisteredControllerMessage(&[0x4A2B_7D64, 0x46845E00])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00])
                .unwrap()
                .channel(),
            u4::new(0xB),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00])
                .unwrap()
                .bank(),
            u7::new(0x7D),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00])
                .unwrap()
                .index(),
            u7::new(0x64),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00])
                .unwrap()
                .controller_data(),
            0x46845E00,
        );
    }

    #[test]
    pub fn data() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00])
                .unwrap()
                .data(),
            &[0x4A2B_7D64, 0x46845E00],
        );
    }
}
