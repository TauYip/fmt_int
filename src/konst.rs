// Stupid workaround before const trait is stabilized.

#![allow(non_snake_case)]

use core::{marker::PhantomData, mem::MaybeUninit};

use crate::{Binary, LowerHex, NumBuffer, NumBufferTrait, Octal, RadixMarker, UpperHex};

#[doc(hidden)]
pub struct NameSpace<Radix: RadixMarker, Int>(PhantomData<(Radix, Int)>);

macro_rules! radix {
    ($radix:ty, $signed:ident and $unsigned:ident, $dig_tab:literal) => {
        impl NameSpace<$radix, $unsigned> {
            pub const fn fmt(
                this: $unsigned,
                mut buf: NumBuffer<$radix, $unsigned>,
            ) -> (NumBuffer<$radix, $unsigned>, usize) {
                const DIG_TAB: &[u8] = $dig_tab;
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf_mut = &mut buf.0;
                let mut offset = buf_mut.len();
                let mut remain = this;
                loop {
                    let digit = remain % BASE;
                    remain /= BASE;
                    offset -= 1;
                    // SAFETY: `remain` will reach 0 and we will break before `offset` wraps
                    unsafe { core::hint::assert_unchecked(offset < buf_mut.len()) }
                    buf_mut[offset].write(DIG_TAB[digit as usize]);
                    if remain == 0 {
                        break;
                    }
                }
                (buf, offset)
            }
        }

        impl NameSpace<$radix, $signed> {
            // Format signed integers in the two's-complement form.
            pub const fn fmt(
                this: $signed,
                buf: NumBuffer<$radix, $signed>,
            ) -> (NumBuffer<$radix, $signed>, usize) {
                // SAFETY:
                // `NumBuffer<$radix, $signed>` and `NumBuffer<$radix, $unsigned>`
                // are the same things.
                let buf: NumBuffer<$radix, $unsigned> = unsafe { core::mem::transmute(buf) };
                let (buf, offset) = NameSpace::<$radix, $unsigned>::fmt(this.cast_unsigned(), buf);
                let buf: NumBuffer<$radix, $signed> = unsafe { core::mem::transmute(buf) };
                (buf, offset)
            }
        }
    };
}

macro_rules! radixes {
    ($signed:ident, $unsigned:ident) => {
        radix! { Binary,   $signed and $unsigned, b"01" }
        radix! { Octal,    $signed and $unsigned, b"01234567" }
        radix! { LowerHex, $signed and $unsigned, b"0123456789abcdef" }
        radix! { UpperHex, $signed and $unsigned, b"0123456789ABCDEF" }
    };
}

radixes! { i8, u8 }
radixes! { i16, u16 }
radixes! { i32, u32 }
radixes! { i64, u64 }
radixes! { i128, u128 }
radixes! { isize, usize }

#[doc(hidden)]
pub struct Decimal;

impl RadixMarker for Decimal {}

macro_rules! decimal {
    ($signed:ident, $unsigned:ident) => {
        impl NumBufferTrait<Decimal> for $unsigned {
            type Buf = [MaybeUninit<u8>; $unsigned::MAX.ilog10() as usize + 1];
            const DEFAULT: Self::Buf = [MaybeUninit::uninit(); _];
        }

        impl NumBufferTrait<Decimal> for $signed {
            type Buf = [MaybeUninit<u8>; $signed::MAX.ilog10() as usize + 1 + 1];
            const DEFAULT: Self::Buf = [MaybeUninit::uninit(); _];
        }

        impl NameSpace<Decimal, $unsigned> {
            pub const fn fmt(
                this: $unsigned,
                mut buf: NumBuffer<Decimal, $unsigned>,
            ) -> (NumBuffer<Decimal, $unsigned>, usize) {
                const DIG_TAB: &[u8] = b"0123456789";
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf_mut = &mut buf.0;
                let mut offset = buf_mut.len();
                let mut remain = this;
                loop {
                    let digit = remain % BASE;
                    remain /= BASE;
                    offset -= 1;
                    // SAFETY: `remain` will reach 0 and we will break before `offset` wraps
                    unsafe { core::hint::assert_unchecked(offset < buf_mut.len()) }
                    buf_mut[offset].write(DIG_TAB[digit as usize]);
                    if remain == 0 {
                        break;
                    }
                }
                (buf, offset)
            }
        }

        impl NameSpace<Decimal, $signed> {
            pub const fn fmt(
                this: $signed,
                mut buf: NumBuffer<Decimal, $signed>,
            ) -> (NumBuffer<Decimal, $signed>, usize) {
                const DIG_TAB: &[u8] = b"0123456789";
                const BASE: $unsigned = DIG_TAB.len() as $unsigned;

                let buf_mut = &mut buf.0;
                let mut offset = buf_mut.len();
                let mut remain = this.unsigned_abs();
                loop {
                    let digit = remain % BASE;
                    remain /= BASE;
                    offset -= 1;
                    // SAFETY: `remain` will reach 0 and we will break before `offset` wraps
                    unsafe { core::hint::assert_unchecked(offset < buf_mut.len()) }
                    buf_mut[offset].write(DIG_TAB[digit as usize]);
                    if remain == 0 {
                        break;
                    }
                }
                if this < 0 {
                    offset -= 1;
                    buf_mut[offset].write(b'-');
                }
                (buf, offset)
            }
        }
    };
}

decimal! { i8, u8 }
decimal! { i16, u16 }
decimal! { i32, u32 }
decimal! { i64, u64 }
decimal! { i128, u128 }
decimal! { isize, usize }

#[macro_export]
#[doc(hidden)]
macro_rules! _const_fmt_int {
    ($num:expr, $num_ty:ty, $radix:ident) => {{
        const DATA: ($crate::NumBuffer<$crate::$radix, $num_ty>, usize) =
            $crate::NameSpace::<$crate::$radix, $num_ty>::fmt($num, $crate::NumBuffer::new());
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

/// Compile-time version of [`fmt_int`](crate::fmt_int).
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
