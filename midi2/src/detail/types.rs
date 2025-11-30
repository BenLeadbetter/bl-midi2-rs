trait Reader {
    fn segment<'a>(&'a self, segment_id: &'static str, index: usize) -> &'a [u8];
    fn segment_mut<'a>(&'a mut self, segment_id: &'static str, index: usize) -> &'a mut [u8];
}

trait Ty {
    fn from_data<R: Reader>(reader: &R) -> Self;
    fn to_data<R: Reader>(&self, reader: &mut R);
    fn validate_data<R: Reader>(_reader: &R) -> Result<(), crate::error::InvalidData> {
        return Ok(());
    }
    fn default_value() -> Self;
}

impl Ty for ux::u4 {
    fn from_data<R: Reader>(_reader: &R) -> Self {
        todo!()
    }
    fn to_data<R: Reader>(&self, _reader: &mut R) {
        todo!()
    }
    fn default_value() -> Self {
        ux::u4::new(0)
    }
}

impl Ty for ux::u7 {
    fn from_data<R: Reader>(_reader: &R) -> Self {
        todo!()
    }
    fn to_data<R: Reader>(&self, _reader: &mut R) {
        todo!()
    }
    fn default_value() -> Self {
        ux::u7::new(0)
    }
}
