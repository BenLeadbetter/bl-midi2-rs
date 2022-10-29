use crate::packet::Packet;

pub mod builder;

mod bounded;
mod slice_data;
mod truncate;

pub use bounded::Bounded;
pub use slice_data::SliceData;
pub use truncate::Truncate;

pub trait MessageTraits {
    const DUMMY: u8 = 0;
}

impl<T> MessageTraits for T where
    T: Clone
        + core::fmt::Debug
        + PartialEq
        + core::convert::TryFrom<Packet>
        + core::convert::Into<Packet>
{
}

#[cfg(test)]
macro_rules! message_traits_test {
    ($t:ty) => {
        use crate::util::MessageTraits;
        #[test]
        fn traits() {
            let _ = <Message as MessageTraits>::DUMMY;
        }
    };
}

#[cfg(test)]
pub(crate) use message_traits_test;
