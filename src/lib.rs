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

/// Compile-time version of [`fmt_int!`](crate::fmt_int).
///
/// # Example
/// ```
/// use fmt_int::const_fmt_int;
/// const N: u8 = 123;
/// assert_eq!(const_fmt_int!(N, u8), format!("{}", N));
/// assert_eq!(const_fmt_int!("{:b}", N, u8), format!("{:b}", N));
/// assert_eq!(const_fmt_int!("{:o}", N, u8), format!("{:o}", N));
/// assert_eq!(const_fmt_int!("{:x}", N, u8), format!("{:x}", N));
/// assert_eq!(const_fmt_int!("{:X}", N, u8), format!("{:X}", N));
/// ```
#[macro_export]
macro_rules! const_fmt_int {
    ($num:expr, $num_ty:ty) => {
        $crate::_const_fmt_int!($num, $num_ty, Decimal)
    };
    ("{:b}", $num:expr, $num_ty:ty) => {
        $crate::_const_fmt_int!($num, $num_ty, Binary)
    };
    ("{:o}", $num:expr, $num_ty:ty) => {
        $crate::_const_fmt_int!($num, $num_ty, Octal)
    };
    ("{:x}", $num:expr, $num_ty:ty) => {
        $crate::_const_fmt_int!($num, $num_ty, LowerHex)
    };
    ("{:X}", $num:expr, $num_ty:ty) => {
        $crate::_const_fmt_int!($num, $num_ty, UpperHex)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _const_fmt_int {
    ($num:expr, $num_ty:ty, $radix:ident) => {{
        const DATA: ($crate::NumBuffer<$crate::$radix, $num_ty>, usize) = {
            let mut buf = $crate::NumBuffer::new();
            let offset = $crate::Const::<$crate::$radix, $num_ty>::fmt($num, &mut buf);
            (buf, offset)
        };
        // Discard uninitialized bytes in `DATA.0`.
        const EXACT_BUF: [u8; DATA.0.0.len() - DATA.1] = unsafe {
            let buf = DATA.0;
            let offset = DATA.1;
            let src = buf.0.as_ptr().add(offset).cast();
            let len = buf.0.len() - offset;
            let mut dest = [0; _];
            core::ptr::copy_nonoverlapping(src, dest.as_mut_ptr(), len);
            dest
        };
        unsafe { str::from_utf8_unchecked(&EXACT_BUF) }
    }};
}

#[cfg(test)]
mod tests;
