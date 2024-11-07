use crate::{buffer::SpecialiseU32, sysex7};

const ERR_EXPECTED_START_OR_COMPLETE: &str = "Expected complete or start packet";
const ERR_EXPECTED_CONTINUE_OR_END: &str = "Expected continue or end packet";

enum State {
    ExpectingStart,
    ExpectingEnd,
    Finished,
}

pub struct Sysex7Parser<B: crate::buffer::BufferMut> {
    state: State,
    index: usize,
    buffer: B,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseResult<'a, B: crate::buffer::BufferMut> {
    Incomplete,
    Complete(sysex7::Sysex7<&'a [B::Unit]>),
}

impl<B: crate::buffer::BufferMut> Sysex7Parser<B> {
    pub fn new() -> Self
    where
        B: crate::buffer::BufferDefault,
    {
        Self {
            buffer: B::default(),
            index: 0,
            state: State::ExpectingStart,
        }
    }

    pub fn new_with_buffer(buffer: B) -> Self {
        Self {
            buffer,
            index: 0,
            state: State::ExpectingStart,
        }
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn buffer(&self) -> &[B::Unit] {
        &self.buffer.buffer()[..self.index]
    }

    pub fn message(&self) -> Option<sysex7::Sysex7<&[B::Unit]>> {
        sysex7::Sysex7::try_from(self.buffer()).ok()
    }

    fn parse_buffer(&self) -> Result<sysex7::Sysex7<&[B::Unit]>, crate::error::InvalidData> {
        sysex7::Sysex7::try_from(self.buffer())
    }

    pub fn parse(
        &mut self,
        packet: sysex7::Packet,
    ) -> Result<ParseResult<B>, crate::error::InvalidData>
    where
        B: crate::buffer::BufferResize,
    {
        use sysex7::packet::Status;

        // if we finished parsing a previous message then we automatically
        // clear it now (we assume it was handled)
        if let State::Finished = self.state {
            self.clear();
            self.state = State::ExpectingStart;
        }

        match self.state {
            State::ExpectingStart => match packet.status() {
                Status::Complete => {
                    self.push(packet);
                    self.state = State::Finished;
                    return Ok(ParseResult::Complete(self.parse_buffer()?));
                }
                Status::Start => {
                    self.push(packet);
                    self.state = State::ExpectingEnd;
                }
                _ => {
                    return Err(crate::error::InvalidData(ERR_EXPECTED_START_OR_COMPLETE));
                }
            },
            State::ExpectingEnd => match packet.status() {
                Status::Continue => {
                    self.push(packet);
                }
                Status::End => {
                    self.push(packet);
                    self.state = State::Finished;
                    return Ok(ParseResult::Complete(self.parse_buffer()?));
                }
                _ => {
                    return Err(crate::error::InvalidData(ERR_EXPECTED_CONTINUE_OR_END));
                }
            },
            State::Finished => unreachable!(),
        }

        Ok(ParseResult::Incomplete)
    }

    fn push(&mut self, packet: sysex7::packet::Packet)
    where
        B: crate::buffer::BufferResize,
    {
        self.buffer.resize(self.buffer().len() + 2);
        self.buffer.buffer_mut().specialise_u32_mut()[self.index..self.index + 2]
            .copy_from_slice(&packet);
        self.index += 2;
    }

    pub fn parse_raw(&mut self, data: &[u32]) -> Result<ParseResult<B>, crate::error::InvalidData>
    where
        B: crate::buffer::BufferResize,
    {
        self.parse(data.try_into()?)
    }
}

impl<B: crate::buffer::BufferMut + crate::buffer::BufferDefault> core::default::Default
    for Sysex7Parser<B>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_raw_trivial_complete() {
        let mut parser = Sysex7Parser::<std::vec::Vec<u32>>::new();
        let buffer = [0x3000_0000, 0x0];
        assert_eq!(
            parser.parse_raw(&buffer[..]),
            Ok(ParseResult::Complete(buffer[..].try_into().unwrap()))
        );
    }

    #[test]
    fn parse_raw_wrong_packet_type() {
        let mut parser = Sysex7Parser::<std::vec::Vec<u32>>::new();
        assert_eq!(
            parser.parse_raw(&[0x2000_0000, 0x0][..]),
            Err(crate::error::InvalidData(
                crate::detail::common_err_strings::ERR_INCORRECT_UMP_MESSAGE_TYPE
            ))
        );
    }

    #[test]
    fn parse_raw_start_with_continue_packet() {
        let mut parser = Sysex7Parser::<std::vec::Vec<u32>>::new();
        assert_eq!(
            parser.parse_raw(&[0x3020_0000, 0x0][..]),
            Err(crate::error::InvalidData(ERR_EXPECTED_START_OR_COMPLETE))
        );
    }

    #[test]
    fn parse_raw_start_with_start_packet() {
        let mut parser = Sysex7Parser::<std::vec::Vec<u32>>::new();
        assert_eq!(
            parser.parse_raw(&[0x3010_0000, 0x0][..]),
            Ok(ParseResult::Incomplete),
        );
    }

    #[test]
    fn parse_raw_start_and_end() {
        let mut parser = Sysex7Parser::<std::vec::Vec<u32>>::new();
        let expected = [0x3010_0000, 0x0, 0x3030_0000, 0x0];
        assert_eq!(
            parser.parse_raw(&[0x3010_0000, 0x0][..]),
            Ok(ParseResult::Incomplete),
        );
        assert_eq!(
            parser.parse_raw(&[0x3030_0000, 0x0][..]),
            Ok(ParseResult::Complete(expected[..].try_into().unwrap())),
        );
    }
}
