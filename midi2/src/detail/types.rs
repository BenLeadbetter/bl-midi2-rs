pub const fn hash(input: &str) -> u32 {
    const_fnv1a_hash::fnv1a_hash_str_32(input)
}

mod buffer {
    use crate::error::BufferOverflow;

    #[allow(private_bounds)]
    pub trait Unit: Copy + UnitPrivate {
        fn zero() -> Self;
    }

    impl Unit for u8 {
        fn zero() -> Self {
            0x0
        }
    }

    impl Unit for u32 {
        fn zero() -> Self {
            0x0
        }
    }

    pub trait Buffer {
        type Unit: Unit;
    }

    pub trait ContiguousBuffer: Buffer {
        fn contiguous_buffer(&self) -> &[Self::Unit];
        fn contiguous_buffer_mut(&mut self) -> &mut [Self::Unit]
        where
            Self: BufferMut;
    }

    pub trait BufferMut {}

    pub trait BufferDefault {
        fn default() -> Self;
    }

    pub trait BufferResize {
        fn resize(&mut self, size: usize);
    }

    pub trait BufferTryResize {
        fn try_resize(&mut self, size: usize) -> Result<(), BufferOverflow>;
    }

    pub trait FromBuffer<T>: Sized {
        fn from_buffer(value: T) -> Self;
    }

    pub trait IntoBuffer<T>: Sized {
        fn into_buffer(self) -> T;
    }

    impl<T, U> IntoBuffer<U> for T
    where
        U: FromBuffer<T>,
    {
        fn into_buffer(self) -> U {
            U::from_buffer(self)
        }
    }

    pub trait TryFromBuffer<T>: Sized {
        fn try_from_buffer(value: T) -> Result<Self, crate::error::BufferOverflow>;
    }

    pub trait TryIntoBuffer<T>: Sized {
        fn try_into_buffer(self) -> Result<T, crate::error::BufferOverflow>;
    }

    impl<T, U> TryIntoBuffer<U> for T
    where
        U: TryFromBuffer<T>,
    {
        fn try_into_buffer(self) -> Result<U, crate::error::BufferOverflow> {
            U::try_from_buffer(self)
        }
    }

    pub trait Ump: Buffer<Unit = u32> {
        fn packet<const L: usize>(&self, index: usize) -> &[u32];
        fn packet_mut<const L: usize>(&mut self, index: usize) -> &mut [u32]
        where
            Self: BufferMut;
    }

    pub trait Bytes: Buffer<Unit = u8> + ContiguousBuffer {
        fn bytes(&self) -> &[u8];
        fn bytes_mut(&mut self) -> &mut [u8]
        where
            Self: BufferMut;
    }

    impl<B: Buffer<Unit = u8> + ContiguousBuffer> Bytes for B {
        fn bytes(&self) -> &[u8] {
            self.contiguous_buffer()
        }
        fn bytes_mut(&mut self) -> &mut [u8]
        where
            Self: BufferMut,
        {
            self.contiguous_buffer_mut()
        }
    }

    impl<U: Unit> Buffer for &[U] {
        type Unit = U;
    }

    impl<U: Unit> ContiguousBuffer for &[U] {
        fn contiguous_buffer(&self) -> &[Self::Unit] {
            self
        }
        fn contiguous_buffer_mut(&mut self) -> &mut [Self::Unit]
        where
            Self: BufferMut,
        {
            unreachable!()
        }
    }

    impl Ump for &[u32] {
        fn packet<const L: usize>(&self, index: usize) -> &[u32] {
            &self[index * L..(index + 1) * L]
        }
        fn packet_mut<const L: usize>(&mut self, _index: usize) -> &mut [u32] {
            unreachable!()
        }
    }

    impl<U: Unit> Buffer for &mut [U] {
        type Unit = U;
    }

    impl<U: Unit> ContiguousBuffer for &mut [U] {
        fn contiguous_buffer(&self) -> &[Self::Unit] {
            self
        }
        fn contiguous_buffer_mut(&mut self) -> &mut [Self::Unit]
        where
            Self: BufferMut,
        {
            self
        }
    }

    impl Ump for &mut [u32] {
        fn packet<const L: usize>(&self, index: usize) -> &[u32] {
            &self[index * L..(index + 1) * L]
        }
        fn packet_mut<const L: usize>(&mut self, index: usize) -> &mut [u32] {
            &mut self[index * L..(index + 1) * L]
        }
    }

    impl<U: Unit> BufferMut for &mut [U] {}

    // impl<const SIZE: usize, U: Unit> Buffer for [U; SIZE] {
    //     type Unit = U;
    //     fn buffer(&self) -> &[Self::Unit] {
    //         &self[..]
    //     }
    // }
    //
    // impl<const SIZE: usize, U: Unit> BufferMut for [U; SIZE] {
    //     fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
    //         &mut self[..]
    //     }
    // }
    //
    // impl<const SIZE: usize, U: Unit> BufferDefault for [U; SIZE] {
    //     fn default() -> Self {
    //         [U::zero(); SIZE]
    //     }
    // }
    //
    // impl<const SIZE: usize, U: Unit> BufferTryResize for [U; SIZE] {
    //     fn try_resize(&mut self, size: usize) -> Result<(), BufferOverflow> {
    //         if size > self.len() {
    //             Err(BufferOverflow)
    //         } else {
    //             Ok(())
    //         }
    //     }
    // }
    //
    // impl<U: Unit> BufferTryResize for &mut [U] {
    //     fn try_resize(&mut self, size: usize) -> Result<(), BufferOverflow> {
    //         if size > self.len() {
    //             Err(BufferOverflow)
    //         } else {
    //             Ok(())
    //         }
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> Buffer for std::vec::Vec<U> {
    //     type Unit = U;
    //     fn buffer(&self) -> &[Self::Unit] {
    //         self
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> BufferMut for std::vec::Vec<U> {
    //     fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
    //         self
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> BufferResize for std::vec::Vec<U> {
    //     fn resize(&mut self, size: usize) {
    //         self.resize(size, U::zero());
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> BufferDefault for std::vec::Vec<U> {
    //     fn default() -> Self {
    //         Default::default()
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> Buffer for &mut std::vec::Vec<U> {
    //     type Unit = U;
    //     fn buffer(&self) -> &[Self::Unit] {
    //         self
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> BufferMut for &mut std::vec::Vec<U> {
    //     fn buffer_mut(&mut self) -> &mut [<Self as Buffer>::Unit] {
    //         self
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> BufferResize for &mut std::vec::Vec<U> {
    //     fn resize(&mut self, size: usize) {
    //         std::vec::Vec::resize(*self, size, U::zero());
    //     }
    // }
    //
    // //
    // // conversion
    // //
    //
    // impl<const SIZE: usize, U: Unit> TryFromBuffer<&[U]> for [U; SIZE] {
    //     fn try_from_buffer(value: &[U]) -> Result<Self, crate::error::BufferOverflow> {
    //         if value.len() > SIZE {
    //             return Err(crate::error::BufferOverflow);
    //         }
    //         let mut buffer = [U::zero(); SIZE];
    //         buffer[..value.len()].copy_from_slice(value);
    //         Ok(buffer)
    //     }
    // }
    //
    // impl<const SIZE: usize, U: Unit> TryFromBuffer<&mut [U]> for [U; SIZE] {
    //     fn try_from_buffer(value: &mut [U]) -> Result<Self, crate::error::BufferOverflow> {
    //         if value.len() > SIZE {
    //             return Err(crate::error::BufferOverflow);
    //         }
    //         let mut buffer = [U::zero(); SIZE];
    //         buffer[..value.len()].copy_from_slice(value);
    //         Ok(buffer)
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<const SIZE: usize, U: Unit> TryFromBuffer<std::vec::Vec<U>> for [U; SIZE] {
    //     fn try_from_buffer(value: std::vec::Vec<U>) -> Result<Self, crate::error::BufferOverflow> {
    //         if value.len() > SIZE {
    //             return Err(crate::error::BufferOverflow);
    //         }
    //         let mut buffer = [U::zero(); SIZE];
    //         buffer[..value.len()].copy_from_slice(&value[..]);
    //         Ok(buffer)
    //     }
    // }
    //
    // impl<'a, U: Unit> FromBuffer<&'a mut [U]> for &'a [U] {
    //     fn from_buffer(value: &'a mut [U]) -> Self {
    //         value
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit, const SIZE: usize> FromBuffer<[U; SIZE]> for std::vec::Vec<U> {
    //     fn from_buffer(value: [U; SIZE]) -> Self {
    //         value.to_vec()
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> FromBuffer<&[U]> for std::vec::Vec<U> {
    //     fn from_buffer(value: &[U]) -> Self {
    //         value.to_vec()
    //     }
    // }
    //
    // #[cfg(any(feature = "std", test))]
    // impl<U: Unit> FromBuffer<&mut [U]> for std::vec::Vec<U> {
    //     fn from_buffer(value: &mut [U]) -> Self {
    //         value.to_vec()
    //     }
    // }

    //
    // private
    //

    pub(crate) const UNIT_ID_U8: u8 = 0;
    pub(crate) const UNIT_ID_U32: u8 = 1;

    pub(crate) trait UnitPrivate: Copy {
        const UNIT_ID: u8;
    }

    impl UnitPrivate for u8 {
        const UNIT_ID: u8 = UNIT_ID_U8;
    }

    impl UnitPrivate for u32 {
        const UNIT_ID: u8 = UNIT_ID_U32;
    }
}

// trait Property<const M: u32, const P: u32, const T: u32>
// where
//     (): Ty<T>,
// {
//     fn read<B: buffer::Buffer>(buffer: &B) -> <() as Ty<T>>::Ty;
//     fn write<B: buffer::Buffer + buffer::BufferMut>(
//         v: &<() as Ty<T>>::Ty,
//         buffer: &mut B,
//     );
// }
//
// impl Property<{ hash("unique_message_id") }, { hash("note_number") }, { hash("u7") }> for () {
//     fn read<B: buffer::Buffer>(buffer: &B) -> <() as Ty<{ hash("u7") }>>::Ty {
//         match <B::Unit as buffer::UnitPrivate>::UNIT_ID {
//             buffer::UNIT_ID_U8 => {
//                 use buffer::SpecialiseU8;
//                 let bytes = buffer.specialise_u8();
//                 <() as Ty<{ hash("u7") }>>::from_data(&bytes[1..2])
//             }
//             buffer::UNIT_ID_U32 => {
//                 use buffer::SpecialiseU32;
//                 let ump = buffer.specialise_u32();
//                 let bytes: &[u8] = bytemuck::cast_slice(ump);
//                 <() as Ty<{ hash("u7") }>>::from_data(&bytes[1..2])
//             }
//             _ => unreachable!(),
//         }
//     }
//     fn write<B: buffer::Buffer + buffer::BufferMut>(
//         v: &<() as Ty<{ hash("u7") }>>::Ty,
//         buffer: &mut B,
//     ) {
//         match <B::Unit as buffer::UnitPrivate>::UNIT_ID {
//             buffer::UNIT_ID_U8 => {
//                 use buffer::SpecialiseU8;
//                 let bytes = buffer.specialise_u8_mut();
//                 <() as Ty<{ hash("u7") }>>::to_data(v, &mut bytes[1..2])
//             }
//             buffer::UNIT_ID_U32 => {
//                 use buffer::SpecialiseU32;
//                 let ump = buffer.specialise_u32_mut();
//                 let bytes: &mut [u8] = bytemuck::cast_slice_mut(ump);
//                 <() as Ty<{ hash("u7") }>>::to_data(v, &mut bytes[1..2])
//             }
//             _ => unreachable!(),
//         }
//     }
// }
//
// trait Ty<const T: u32> {
//     type Ty;
//     fn from_data(data: &[u8]) -> Self::Ty;
//     fn to_data(ty: &Self::Ty, data: &mut [u8]);
//     fn validate_value(_value: &Self::Ty) -> Result<(), crate::error::InvalidData> {
//         return Ok(());
//     }
//     fn validate_data(_data: &mut [u8]) -> Result<(), crate::error::InvalidData> {
//         return Ok(());
//     }
//     fn default_value() -> Self::Ty;
//     // TODO: resize? perhaps just a size hint could suffice?
// }
//
// impl Ty<{ hash("u4") }> for () {
//     type Ty = ux::u4;
//     fn from_data(data: &[u8]) -> Self::Ty {
//         debug_assert!(data.len() == 1);
//         ux::u4::new(data[0])
//     }
//     fn to_data(v: &Self::Ty, data: &mut [u8]) {
//         debug_assert!(data.len() == 1);
//         data[0] = (*v).into();
//     }
//     fn default_value() -> Self::Ty {
//         ux::u4::new(0)
//     }
// }
//
// impl Ty<{ hash("u7") }> for () {
//     type Ty = ux::u7;
//     fn from_data(data: &[u8]) -> Self::Ty {
//         debug_assert!(data.len() == 1);
//         ux::u7::new(data[0] & 0b01111111)
//     }
//     fn to_data(v: &Self::Ty, data: &mut [u8]) {
//         debug_assert!(data.len() == 1);
//         data[0] &= 0b10000000;
//         data[0] |= u8::from(*v);
//     }
//     fn default_value() -> Self::Ty {
//         ux::u7::new(0)
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn read_note_number_from_ump() {
//         let ump = [0x2D9E_753D_u32, 0x0, 0x0, 0x0];
//         let note_number = <() as Property<
//             { hash("unique_message_id") },
//             { hash("note_number") },
//             { hash("u7") },
//         >>::read(&ump);
//         assert_eq!(note_number, ux::u7::new(0x75));
//     }
//
//     #[test]
//     fn read_note_number_from_bytes() {
//         let bytes = [0x9E_u8, 0x75, 0x3D];
//         let note_number = <() as Property<
//             { hash("unique_message_id") },
//             { hash("note_number") },
//             { hash("u7") },
//         >>::read(&bytes);
//         assert_eq!(note_number, ux::u7::new(0x75));
//     }
//
//     #[test]
//     fn write_note_number_from_ump() {
//         let mut ump = [0x2D9E_753D_u32, 0x0, 0x0, 0x0];
//         <() as Property<
//             { hash("unique_message_id") },
//             { hash("note_number") },
//             { hash("u7") },
//         >>::write(&ux::u7::new(0x22), &mut ump);
//         assert_eq!(&ump[..], &[0x2D9E_223D_u32, 0x0, 0x0, 0x0][..]);
//     }
//
//     #[test]
//     fn write_note_number_from_bytes() {
//         let mut bytes = [0x9E_u8, 0x75, 0x3D];
//         <() as Property<
//             { hash("unique_message_id") },
//             { hash("note_number") },
//             { hash("u7") },
//         >>::write(&ux::u7::new(0x22), &mut bytes);
//         assert_eq!(&bytes[..], &[0x9E_u8, 0x22, 0x3D][..]);
//     }
// }
