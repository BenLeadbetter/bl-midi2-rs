#![no_std]
#![doc = include_str!("../README.md")]

pub mod ci;
pub mod message;

mod error;
mod result;
mod traits;
mod util;

pub use error::*;
pub use result::*;

// forward expose some numeric types from the
// 3rd party ux crate
#[rustfmt::skip]
pub use ux:: {
    u1, u2, u3, u4, u5, u6, u7, u9, u10, u11, u12,
    u13, u14, u15, u17, u18, u19, u20, u21, u22,
    u23, u24, u25, u26, u27, u28, u29, u30, u31,
};
