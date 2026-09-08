#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub mod radix;

pub use radix::*;

/// See [crate level documentation](self) for more information.
#[macro_export]
macro_rules! fmt_int {
    ($num:expr) => {
        $num.format_into(&mut core::fmt::NumBuffer::new())
    };
    ("{:b}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::Binary, _>::new())
    };
    ("{:o}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::Octal, _>::new())
    };
    ("{:x}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::LowerHex, _>::new())
    };
    ("{:X}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::UpperHex, _>::new())
    };
}

#[cfg(test)]
mod tests;
