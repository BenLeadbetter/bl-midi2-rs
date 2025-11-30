pub const fn hash(input: &str) -> u32 {
    const_fnv1a_hash::fnv1a_hash_str_32(input)
}

trait Property<const M: u32, const P: u32, const T: u32>
where
    (): Ty<T>,
{
    fn read<B: crate::buffer::Buffer>(buffer: &B) -> <() as Ty<T>>::Ty;
    fn write<B: crate::buffer::Buffer + crate::buffer::BufferMut>(
        v: &<() as Ty<T>>::Ty,
        buffer: &mut B,
    );
}

impl Property<{ hash("channel_voice1_note_on") }, { hash("note_number") }, { hash("u7") }> for () {
    fn read<B: crate::buffer::Buffer>(buffer: &B) -> <() as Ty<{ hash("u7") }>>::Ty {
        use crate::buffer;
        match <B::Unit as buffer::UnitPrivate>::UNIT_ID {
            buffer::UNIT_ID_U8 => {
                use crate::buffer::SpecialiseU8;
                let bytes = buffer.specialise_u8();
                <() as Ty<{ hash("u7") }>>::from_data(&bytes[1..2])
            }
            buffer::UNIT_ID_U32 => {
                use crate::buffer::SpecialiseU32;
                let ump = buffer.specialise_u32();
                let bytes: &[u8] = bytemuck::cast_slice(ump);
                <() as Ty<{ hash("u7") }>>::from_data(&bytes[1..2])
            }
            _ => unreachable!(),
        }
    }
    fn write<B: crate::buffer::Buffer + crate::buffer::BufferMut>(
        v: &<() as Ty<{ hash("u7") }>>::Ty,
        buffer: &mut B,
    ) {
        use crate::buffer;
        match <B::Unit as buffer::UnitPrivate>::UNIT_ID {
            buffer::UNIT_ID_U8 => {
                use crate::buffer::SpecialiseU8;
                let bytes = buffer.specialise_u8_mut();
                <() as Ty<{ hash("u7") }>>::to_data(v, &mut bytes[1..2])
            }
            buffer::UNIT_ID_U32 => {
                use crate::buffer::SpecialiseU32;
                let ump = buffer.specialise_u32_mut();
                let bytes: &mut [u8] = bytemuck::cast_slice_mut(ump);
                <() as Ty<{ hash("u7") }>>::to_data(v, &mut bytes[1..2])
            }
            _ => unreachable!(),
        }
    }
}

trait Ty<const T: u32> {
    type Ty;
    fn from_data(data: &[u8]) -> Self::Ty;
    fn to_data(ty: &Self::Ty, data: &mut [u8]);
    fn validate_value(_value: &Self::Ty) -> Result<(), crate::error::InvalidData> {
        return Ok(());
    }
    fn validate_data(_data: &mut [u8]) -> Result<(), crate::error::InvalidData> {
        return Ok(());
    }
    fn default_value() -> Self::Ty;
    // TODO: resize? perhaps just a size hint could suffice?
}

impl Ty<{ hash("u4") }> for () {
    type Ty = ux::u4;
    fn from_data(data: &[u8]) -> Self::Ty {
        debug_assert!(data.len() == 1);
        ux::u4::new(data[0])
    }
    fn to_data(v: &Self::Ty, data: &mut [u8]) {
        debug_assert!(data.len() == 1);
        data[0] = (*v).into();
    }
    fn default_value() -> Self::Ty {
        ux::u4::new(0)
    }
}

impl Ty<{ hash("u7") }> for () {
    type Ty = ux::u7;
    fn from_data(data: &[u8]) -> Self::Ty {
        debug_assert!(data.len() == 1);
        ux::u7::new(data[0] & 0b01111111)
    }
    fn to_data(v: &Self::Ty, data: &mut [u8]) {
        debug_assert!(data.len() == 1);
        data[0] &= 0b10000000;
        data[0] |= u8::from(*v);
    }
    fn default_value() -> Self::Ty {
        ux::u7::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_note_number_from_ump() {
        let ump = [0x2D9E_753D_u32, 0x0, 0x0, 0x0];
        let note_number = <() as Property<
            { hash("channel_voice1_note_on") },
            { hash("note_number") },
            { hash("u7") },
        >>::read(&ump);
        assert_eq!(note_number, ux::u7::new(0x75));
    }

    #[test]
    fn read_note_number_from_bytes() {
        let bytes = [0x9E_u8, 0x75, 0x3D];
        let note_number = <() as Property<
            { hash("channel_voice1_note_on") },
            { hash("note_number") },
            { hash("u7") },
        >>::read(&bytes);
        assert_eq!(note_number, ux::u7::new(0x75));
    }

    #[test]
    fn write_note_number_from_ump() {
        let mut ump = [0x2D9E_753D_u32, 0x0, 0x0, 0x0];
        <() as Property<
            { hash("channel_voice1_note_on") },
            { hash("note_number") },
            { hash("u7") },
        >>::write(&ux::u7::new(0x22), &mut ump);
        assert_eq!(&ump[..], &[0x2D9E_223D_u32, 0x0, 0x0, 0x0][..]);
    }

    #[test]
    fn write_note_number_from_bytes() {
        let mut bytes = [0x9E_u8, 0x75, 0x3D];
        <() as Property<
            { hash("channel_voice1_note_on") },
            { hash("note_number") },
            { hash("u7") },
        >>::write(&ux::u7::new(0x22), &mut bytes);
        assert_eq!(&bytes[..], &[0x9E_u8, 0x22, 0x3D][..]);
    }
}
