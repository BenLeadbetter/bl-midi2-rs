use crate::{buffer, error, sysex7};

pub struct Sysex7Accumulator<B: buffer::Buffer + buffer::BufferMut> {
    buffer: B,
    end: usize,
}

impl<B: buffer::Buffer + buffer::BufferMut> Sysex7Accumulator<B> {
    fn empty(&self) -> bool {
        self.end == 0
    }

    fn reset(&mut self) {
        self.end = 0;
    }

    pub fn new(mut buffer: B) -> Self {
        buffer.buffer_mut().fill(<B::Unit as buffer::Unit>::zero());
        Self { buffer, end: 0 }
    }

    pub fn feed(&mut self, data: &[B::Unit]) -> Result<(), error::InvalidData>
    where
        B: buffer::BufferResize,
    {
        match <B::Unit as buffer::UnitPrivate>::UNIT_ID {
            buffer::UNIT_ID_U8 => feed_bytes(self, data),
            buffer::UNIT_ID_U32 => feed_ump(self, data),
            _ => unreachable!(),
        }
    }

    pub fn try_feed(&mut self, data: &[B::Unit]) -> Result<(), error::Error>
    where
        B: buffer::BufferTryResize,
    {
        todo!()
    }
}

fn feed_bytes<B: buffer::Buffer + buffer::BufferMut + buffer::BufferResize>(
    reader: &mut Sysex7Accumulator<B>,
    data: &[B::Unit],
) -> Result<(), error::InvalidData> {
    use crate::buffer::{SpecialiseU8, UnitPrivate};

    if data.is_empty() {
        return Ok(());
    }

    let mut buffer = reader.buffer.specialise_u8_mut();
    let data = UnitPrivate::specialise_buffer_u8(data);

    for byte in data {
        if reader.empty() && byte != sysex7::START_BYTE {
            continue;
            return Err(error::InvalidData("Expected SysEx start byte (0xF0)"));
        }

        if data
            .iter()
            .any(|byte| 0b1000_0000 & *byte != 0x0 && *byte != 0xF7)
        {
            reader.reset();
            return Err(error::InvalidData("Expected SysEx start byte (0xF0)"));
        }
    }

    todo!()
}

fn feed_ump<B: buffer::Buffer + buffer::BufferMut + buffer::BufferResize>(
    reader: &mut Sysex7Accumulator<B>,
    data: &[B::Unit],
) -> Result<(), error::InvalidData> {
    todo!()
}
