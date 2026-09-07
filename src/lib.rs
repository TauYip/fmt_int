//! Format integers on stack
//! in binary, octal, decimal or hexadecimal.
//!
//! # Examples
//! ```
//! use fmt_int::{FmtInto, fmt_int};
//!
//! fn main() {
//!     let num: u8 = 123;
//!     assert_eq!(fmt_int!(num), format!("{}", num));
//!     assert_eq!(fmt_int!("{:b}", num), format!("{:b}", num));
//!     assert_eq!(fmt_int!("{:o}", num), format!("{:o}", num));
//!     assert_eq!(fmt_int!("{:x}", num), format!("{:x}", num));
//!     assert_eq!(fmt_int!("{:X}", num), format!("{:X}", num));
//! }
//! ```
//! Or you can do it explicitly
//! ```
//! use fmt_int::{FmtInto, NumBuffer, binary};
//!
//! fn main() {
//!     let mut buf = NumBuffer::<binary, _>::new();
//!     _ = 123u8.fmt_into(&mut buf);
//! }
//! ```
//!
//! # Advantage
//! Currently the standard library doesn't decouple
//! the formatting of integers (in binary, octal or hexadecimal)
//! with [`Formatter`](core::fmt::Formatter),
//! which sometimes causes performance penalty.
//!
//! With this crate, you can do
//! ```
//! # use fmt_int::*;
//! ["bytes: ", fmt_int!("{:b}", 123u8)].concat();
//! ```
//! which is faster and more memory efficient than
//! ```
//! format!("bytes: {:b}", 123);
//! ```
//!
//! # Similar crates
//! - [`numtoa`](https://docs.rs/numtoa)
//! - Fast floats formatting: [`zmij`](https://docs.rs/zmij)

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
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::binary, _>::new())
    };
    ("{:o}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::octal, _>::new())
    };
    ("{:x}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::lowerhex, _>::new())
    };
    ("{:X}", $num:expr) => {
        $num.fmt_into(&mut $crate::NumBuffer::<$crate::upperhex, _>::new())
    };
}

#[cfg(test)]
mod tests;
