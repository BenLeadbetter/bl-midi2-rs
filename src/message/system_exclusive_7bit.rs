use crate::{
    error::Error,
    result::Result,
    message::helpers as message_helpers,
    util::{BitOps, Truncate, debug},
};

pub struct PayloadIterator<'a> {
    data: &'a [u32],
    index: usize,
    total: usize,
}

impl<'a> core::iter::Iterator for PayloadIterator<'a> {
    type Item = ux::u7;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.total {
            None
        } else {
            let ret: ux::u7 = self.data[(self.index + 2) / 4].octet((self.index + 2) % 4).truncate();
            self.index += 1;
            Some(ret)
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Sysex7Message<'a>(&'a [u32]);

impl<'a> Sysex7Message<'a> {
    const OP_CODE: ux::u4 = ux::u4::new(0x3);
    pub fn builder(buffer: &'a mut [u32]) -> Sysex7MessageBuilder<'a> {
        Sysex7MessageBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn status(&self) -> Status {
        status_from_packet(self.0).expect("valid status")
    }
    pub fn payload(&self) -> PayloadIterator {
        PayloadIterator{
            data: self.0,
            index: 0,
            total: u32::from(self.0[0].nibble(3)) as usize,
        }
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        validate_buffer(data)?;
        validate_type(data)?;
        status_from_packet(data)?;
        validate_data(data)?;
        Ok(Sysex7Message(&data[..2]))
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
}

debug::message_debug_impl!(Sysex7Message);

enum BuilderImpl<'a> {
    Ok(&'a mut [u32]),
    Err(Error),
}

pub struct Sysex7MessageBuilder<'a>(BuilderImpl<'a>);

impl<'a> Sysex7MessageBuilder<'a> {
    pub fn group(&mut self, g: ux::u4) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
    pub fn status(&mut self, s: Status) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            buffer[0].set_nibble(
                2,
                match s {
                    Status::Complete => ux::u4::new(0x0),
                    Status::Begin => ux::u4::new(0x1),
                    Status::Continue => ux::u4::new(0x2),
                    Status::End => ux::u4::new(0x3),
                },
            );
        }
        self
    }
    pub fn payload<'b, I: core::iter::Iterator<Item = &'b ux::u7>>(&mut self, mut data: I) -> &mut Self {
        if let BuilderImpl::Ok(buffer) = &mut self.0 {
            let mut count = 0_u8;
            for i in 0_usize..6_usize {
                if let Some(&v) = data.next() {
                    buffer[(i + 2) / 4].set_octet((i + 2) % 4, v.into());
                    count += 1;
                } else {
                    break;
                }
            }
            if data.next().is_some() {
                self.0 = BuilderImpl::Err(Error::InvalidData);
            } else {
                buffer[0].set_nibble(3, count.truncate());
            }
        }
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        if buffer.len() >= 2 {
            let buffer = &mut buffer[..2];
            for v in buffer.iter_mut() {
                *v = 0;
            }
            message_helpers::write_type_to_packet(Sysex7Message::OP_CODE, buffer);
            Self(BuilderImpl::Ok(buffer))
        } else {
            Self(BuilderImpl::Err(Error::BufferOverflow))
        }
    }
    pub fn build(&'a self) -> Result<Sysex7Message<'a>> {
        match &self.0 {
            BuilderImpl::Ok(buffer) => Ok(Sysex7Message(buffer)),
            BuilderImpl::Err(e) => Err(e.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Complete,
    Begin,
    Continue,
    End,
}

impl core::default::Default for Status {
    fn default() -> Self {
        Status::Complete
    }
}

fn validate_type(p: &[u32]) -> Result<()> {
    if p[0].nibble(0) != Sysex7Message::OP_CODE {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

fn status_from_packet(p: &[u32]) -> Result<Status> {
    match u8::from(p[0].nibble(2)) {
        0x0 => Ok(Status::Complete),
        0x1 => Ok(Status::Begin),
        0x2 => Ok(Status::Continue),
        0x3 => Ok(Status::End),
        _ => Err(Error::InvalidData),
    }
}

fn validate_buffer(buffer: &[u32]) -> Result<()> {
    if buffer.len() >= 2 {
        Ok(())
    } else {
        Err(Error::BufferOverflow)
    }
}

fn validate_data(p: &[u32]) -> Result<()> {
    let n: usize = u8::from(p[0].nibble(3)).into();
    if n > 6 {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub struct Sysex7MessageGroup<'a>(&'a [u32]);

impl<'a> core::fmt::Debug for Sysex7MessageGroup<'a> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.write_fmt(format_args!("{}(", "Sysex7MessageGroup"))?;
        for chunk in self.0.chunks_exact(2) {
            Sysex7Message(chunk).fmt(fmt)?;
            fmt.write_str(", ")?;
        }
        fmt.write_str(")")
    }
}

impl<'a> Sysex7MessageGroup<'a> {
    pub fn builder(buffer: &mut [u32]) -> Sysex7MessageGroupBuilder<'a> {
        Sysex7MessageGroupBuilder::new(buffer)
    }
    pub fn group(&self) -> ux::u4 {
        message_helpers::group_from_packet(self.0)
    }
    pub fn messages(&self) -> Sysex7MessageGroupIterator<'a> {
        Sysex7MessageGroupIterator(self.0.chunks_exact(2))        
    }
}

pub struct Sysex7MessageGroupIterator<'a>(core::slice::ChunksExact<'a, u32>);

impl<'a> core::iter::Iterator for Sysex7MessageGroupIterator<'a> {
    type Item = Sysex7Message<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Sysex7Message)
    }
}

pub struct Sysex7MessageGroupBuilder<'a>(&'a mut [u32]);

impl<'a> Sysex7MessageGroupBuilder<'a> {
    pub fn new(buffer: &mut [u32]) -> Self {
        todo!()
    }
    pub fn build(&self) -> Sysex7MessageGroup<'a> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_message_type() {
        assert_eq!(
            Sysex7Message::from_data(&[0x2000_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn invalid_status_bit() {
        assert_eq!(
            Sysex7Message::from_data(&[0x30A0_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn data_overflow() {
        assert_eq!(
            Sysex7Message::from_data(&[0x3009_0000, 0x0]),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn builder() {
        assert_eq!(
            Sysex7Message::builder(&mut [0x0, 0x0])
                .group(ux::u4::new(0x1))
                .status(Status::Begin)
                .payload([ux::u7::new(0x12), ux::u7::new(0x34), ux::u7::new(0x56),].iter())
                .build(),
            Ok(Sysex7Message(&[0x3113_1234, 0x5600_0000,])),
        );
    }

    #[test]
    fn builder_invalid_payload() {
        assert_eq!(
            Sysex7Message::builder(&mut [0x0, 0x0])
                .payload([ux::u7::default(); 7].iter())
                .build(),
            Err(Error::InvalidData),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Sysex7Message::from_data(&[0x3C00_0000, 0x0,]).unwrap().group(),
            ux::u4::new(0xC),
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            Sysex7Message::from_data(&[0x3020_0000, 0x0,]).unwrap().status(),
            Status::Continue,
        );
    }

    #[test]
    fn data() {
        assert_eq!(Sysex7Message::from_data(&[0x3004_1234, 0x5678_0000,]).unwrap().data(), &[0x30041234, 0x5678_0000]);
    }

    #[test]
    fn payload() {
        let message = Sysex7Message::from_data(&[0x3004_1234, 0x5678_0000,]).unwrap();
        let mut buffer = [ux::u7::new(0); 4];
        for (i, v) in message.payload().enumerate() {
            buffer[i] = v;
        }
        assert_eq!(&buffer, &[ux::u7::new(0x12), ux::u7::new(0x34), ux::u7::new(0x56), ux::u7::new(0x78)]);
    }
}
